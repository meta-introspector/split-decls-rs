macro_rules! convert_param_list_to_arg_list {
    () => {
        # [doc = " Convert a list of function params to a list of arguments that can be passed"] # [doc = " into a function call."] pub (crate) fn convert_param_list_to_arg_list (list : ast :: ParamList) -> ast :: ArgList { let mut args = vec ! [] ; for param in list . params () { if let Some (ast :: Pat :: IdentPat (pat)) = param . pat () && let Some (name) = pat . name () { let name = name . to_string () ; let expr = make :: expr_path (make :: ext :: ident_path (& name)) ; args . push (expr) ; } } make :: arg_list (args) }
    };
}

convert_param_list_to_arg_list!()