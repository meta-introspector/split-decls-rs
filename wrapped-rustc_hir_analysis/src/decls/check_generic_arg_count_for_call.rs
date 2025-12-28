macro_rules! deps {
    () => {
        GenericArgCountResult!();
        HirTyLowerer!();
        GenericArgPosition!();
        IsMethodCall!();
    };
}

macro_rules! check_generic_arg_count_for_call {
    () => {
        deps!();
        # [doc = " Checks that the correct number of generic arguments have been provided."] # [doc = " Used specifically for function calls."] pub fn check_generic_arg_count_for_call (cx : & dyn HirTyLowerer < '_ > , def_id : DefId , generics : & ty :: Generics , seg : & hir :: PathSegment < '_ > , is_method_call : IsMethodCall ,) -> GenericArgCountResult { let gen_pos = match is_method_call { IsMethodCall :: Yes => GenericArgPosition :: MethodCall , IsMethodCall :: No => GenericArgPosition :: Value , } ; let has_self = generics . parent . is_none () && generics . has_self ; check_generic_arg_count (cx , def_id , seg , generics , gen_pos , has_self) }
    };
}

check_generic_arg_count_for_call!()