use rustc_driver::{Callbacks, Compilation, RunCompiler};
use rustc_interface::{interface, Queries};
use rustc_middle::ty::TyCtxt;
use rustc_hir as hir;
use std::process;

struct HirTracer;

impl Callbacks for HirTracer {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            println!("🔍 HIR Analysis Started");
            
            // Access the HIR crate
            let hir_crate = tcx.hir().crate_root();
            println!("📊 HIR Crate: {:?}", hir_crate);
            
            // Walk through all items in the crate
            tcx.hir().visit_all_item_likes_in_crate(&mut HirVisitor { tcx });
            
            println!("✅ HIR Analysis Complete");
        });
        
        Compilation::Continue
    }
}

struct HirVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> hir::intravisit::Visitor<'tcx> for HirVisitor<'tcx> {
    type NestedFilter = rustc_middle::hir::nested_filter::All;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.tcx.hir()
    }

    fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) {
        println!("🔍 HIR Item: {} (kind: {:?})", item.ident, item.kind);
        
        match &item.kind {
            hir::ItemKind::Fn(sig, _, body_id) => {
                println!("  📝 Function: {}", item.ident);
                println!("    Signature: {:?}", sig);
                
                let body = self.tcx.hir().body(*body_id);
                println!("    Body: {:?}", body.value);
            }
            _ => {}
        }
        
        hir::intravisit::walk_item(self, item);
    }

    fn visit_expr(&mut self, expr: &'tcx hir::Expr<'tcx>) {
        println!("  🔍 Expression: {:?} at {:?}", expr.kind, expr.span);
        hir::intravisit::walk_expr(self, expr);
    }
}

pub fn run_with_hir_tracing(args: Vec<String>) -> i32 {
    let mut callbacks = HirTracer;
    
    let mut compiler = RunCompiler::new(&args, &mut callbacks);
    compiler.run().unwrap_or_else(|_| process::exit(1));
    
    0
}
