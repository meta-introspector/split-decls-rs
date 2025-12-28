macro_rules! deps {
    () => {
        LateContext!();
        UndefinedTransmuteLint!();
    };
}

macro_rules! check_ptr_transmute_in_const {
    () => {
        deps!();
        # [doc = " Check for transmutes that exhibit undefined behavior."] # [doc = " For example, transmuting pointers to integers in a const context."] # [doc = ""] # [doc = " Why do we consider const functions and associated constants only?"] # [doc = ""] # [doc = " Generally, undefined behavior in const items are handled by the evaluator."] # [doc = " But, const functions and associated constants are evaluated only when referenced."] # [doc = " This can result in undefined behavior in a library going unnoticed until"] # [doc = " the function or constant is actually used."] # [doc = ""] # [doc = " Therefore, we only consider const functions and associated constants here and leave"] # [doc = " other const items to be handled by the evaluator."] fn check_ptr_transmute_in_const < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > , body_owner_def_id : LocalDefId , const_context : Option < hir :: ConstContext > , src : Ty < 'tcx > , dst : Ty < 'tcx > ,) { if matches ! (const_context , Some (hir :: ConstContext :: ConstFn)) || matches ! (cx . tcx . def_kind (body_owner_def_id) , DefKind :: AssocConst) { if src . is_raw_ptr () && dst . is_integral () { cx . tcx . emit_node_span_lint (PTR_TO_INTEGER_TRANSMUTE_IN_CONSTS , expr . hir_id , expr . span , UndefinedTransmuteLint ,) ; } } }
    };
}

check_ptr_transmute_in_const!();