macro_rules! get_lib_prefix {
    () => {
        # [doc = " See [`get_lib_filename`] for more details"] pub fn get_lib_prefix (kind : & str) -> & str { match kind { "lib" | "rlib" => "lib" , "staticlib" | "dylib" | "proc-macro" => { if cfg ! (windows) { "" } else { "lib" } } _ => unreachable ! () , } }
    };
}

get_lib_prefix!();