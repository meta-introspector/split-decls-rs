mkuse!{use rustc_hir :: { Expr , ExprKind } ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use crate :: LateContext ;}

macro_rules! peel_casts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function peel_casts in module {}", module_path!());
    };
}

mkfn!{
    peel_casts_introspect!();
    # [doc = " Given an expression, peel all of casts (`<expr> as ...`, `<expr>.cast{,_mut,_const}()`,"] # [doc = " `ptr::from_ref(<expr>)`, ...) and init expressions."] # [doc = ""] # [doc = " Returns the innermost expression and a boolean representing if one of the casts was"] # [doc = " `UnsafeCell::raw_get(<expr>)`"] pub (crate) fn peel_casts < 'tcx > (cx : & LateContext < 'tcx > , mut e : & 'tcx Expr < 'tcx > ,) -> (& 'tcx Expr < 'tcx > , bool) { let mut gone_trough_unsafe_cell_raw_get = false ; loop { e = e . peel_blocks () ; e = if let ExprKind :: Cast (expr , _) = e . kind { expr } else if let ExprKind :: MethodCall (_ , expr , [] , _) = e . kind && let Some (def_id) = cx . typeck_results () . type_dependent_def_id (e . hir_id) && matches ! (cx . tcx . get_diagnostic_name (def_id) , Some (sym :: ptr_cast | sym :: const_ptr_cast | sym :: ptr_cast_mut | sym :: ptr_cast_const)) { expr } else if let ExprKind :: Call (path , [arg]) = e . kind && let ExprKind :: Path (ref qpath) = path . kind && let Some (def_id) = cx . qpath_res (qpath , path . hir_id) . opt_def_id () && matches ! (cx . tcx . get_diagnostic_name (def_id) , Some (sym :: ptr_from_ref | sym :: unsafe_cell_raw_get | sym :: transmute)) { if cx . tcx . is_diagnostic_item (sym :: unsafe_cell_raw_get , def_id) { gone_trough_unsafe_cell_raw_get = true ; } arg } else { let init = cx . expr_or_init (e) ; if init . hir_id != e . hir_id { init } else { break ; } } ; } (e , gone_trough_unsafe_cell_raw_get) }
}