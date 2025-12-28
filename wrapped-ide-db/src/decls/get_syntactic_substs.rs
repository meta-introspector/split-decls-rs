macro_rules! deps {
    () => {
        AstSubsts!();
    };
}

macro_rules! get_syntactic_substs {
    () => {
        deps!();
        fn get_syntactic_substs (impl_def : ast :: Impl) -> Option < AstSubsts > { let target_trait = impl_def . trait_ () ? ; let path_type = match target_trait { ast :: Type :: PathType (path) => path , _ => return None , } ; let generic_arg_list = path_type . path () ? . segment () ? . generic_arg_list () ? ; get_type_args_from_arg_list (generic_arg_list) }
    };
}

get_syntactic_substs!()