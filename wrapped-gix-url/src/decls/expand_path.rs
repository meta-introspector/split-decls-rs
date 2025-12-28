macro_rules! deps {
    () => {
        ForUser!();
        Error!();
    };
}

macro_rules! expand_path {
    () => {
        deps!();
        # [doc = " Expand `path` for the given `user`, which can be obtained by [`parse()`], resolving the home directories"] # [doc = " of `user` automatically."] # [doc = ""] # [doc = " If more precise control of the resolution mechanism is needed, then use the [expand_path::with()] function."] pub fn expand_path (user : Option < & expand_path :: ForUser > , path : & BStr) -> Result < PathBuf , expand_path :: Error > { expand_path :: with (user , path , | user | match user { expand_path :: ForUser :: Current => gix_path :: env :: home_dir () , expand_path :: ForUser :: Name (user) => { gix_path :: env :: home_dir () . and_then (| home | home . parent () . map (| home_dirs | home_dirs . join (user . to_string ()))) } }) }
    };
}

expand_path!();