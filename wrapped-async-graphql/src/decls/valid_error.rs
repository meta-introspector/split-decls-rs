macro_rules! deps {
    () => {
        QueryPathNode!();
    };
}

macro_rules! valid_error {
    () => {
        deps!();
        fn valid_error (path_node : & QueryPathNode , msg : String) -> String { format ! ("\"{}\", {}" , path_node , msg) }
    };
}

valid_error!();