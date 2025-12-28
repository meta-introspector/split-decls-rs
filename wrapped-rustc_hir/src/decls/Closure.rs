macro_rules! deps {
    () => {
        GenericParam!();
        BodyId!();
        ClosureKind!();
        Constness!();
        FnDecl!();
        ClosureBinder!();
    };
}

macro_rules! Closure {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Closure < 'hir > { pub def_id : LocalDefId , pub binder : ClosureBinder , pub constness : Constness , pub capture_clause : CaptureBy , pub bound_generic_params : & 'hir [GenericParam < 'hir >] , pub fn_decl : & 'hir FnDecl < 'hir > , pub body : BodyId , # [doc = " The span of the declaration block: 'move |...| -> ...'"] pub fn_decl_span : Span , # [doc = " The span of the argument block `|...|`"] pub fn_arg_span : Option < Span > , pub kind : ClosureKind , }
    };
}

Closure!();