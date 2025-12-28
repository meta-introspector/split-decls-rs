macro_rules! path_contains_type_arguments {
    () => {
        # [doc = " Returns whether `path` or any of its qualifiers contains type arguments."] fn path_contains_type_arguments (path : Option < ast :: Path >) -> bool { if let Some (path) = path { if let Some (segment) = path . segment () && segment . generic_arg_list () . is_some () { cov_mark :: hit ! (type_arguments_within_path) ; return true ; } return path_contains_type_arguments (path . qualifier ()) ; } false }
    };
}

path_contains_type_arguments!();