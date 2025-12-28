macro_rules! push_generic_params_internal {
    () => {
        fn push_generic_params_internal < 'tcx > (tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > , output : & mut String , visited : & mut FxHashSet < Ty < 'tcx > > ,) -> bool { assert_eq ! (args , tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , args)) ; let mut args = args . non_erasable_generics () . peekable () ; if args . peek () . is_none () { return false ; } let cpp_like_debuginfo = cpp_like_debuginfo (tcx) ; output . push ('<') ; for type_parameter in args { match type_parameter { GenericArgKind :: Type (type_parameter) => { push_debuginfo_type_name (tcx , type_parameter , true , output , visited) ; } GenericArgKind :: Const (ct) => { push_const_param (tcx , ct , output) ; } other => bug ! ("Unexpected non-erasable generic: {:?}" , other) , } push_arg_separator (cpp_like_debuginfo , output) ; } pop_arg_separator (output) ; push_close_angle_bracket (cpp_like_debuginfo , output) ; true }
    };
}

push_generic_params_internal!()