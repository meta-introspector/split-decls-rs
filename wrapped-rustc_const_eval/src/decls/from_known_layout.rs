macro_rules! from_known_layout {
    () => {
        # [doc = " Use the already known layout if given (but sanity check in debug mode),"] # [doc = " or compute the layout."] # [cfg_attr (not (debug_assertions) , inline (always))] pub (super) fn from_known_layout < 'tcx > (tcx : TyCtxtAt < 'tcx > , typing_env : TypingEnv < 'tcx > , known_layout : Option < TyAndLayout < 'tcx > > , compute : impl FnOnce () -> InterpResult < 'tcx , TyAndLayout < 'tcx > > ,) -> InterpResult < 'tcx , TyAndLayout < 'tcx > > { match known_layout { None => compute () , Some (known_layout) => { if cfg ! (debug_assertions) { let check_layout = compute () ? ; if ! mir_assign_valid_types (tcx . tcx , typing_env , check_layout , known_layout) { span_bug ! (tcx . span , "expected type differs from actual type.\nexpected: {}\nactual: {}" , known_layout . ty , check_layout . ty ,) ; } } interp_ok (known_layout) } } }
    };
}

from_known_layout!()