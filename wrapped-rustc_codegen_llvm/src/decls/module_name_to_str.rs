macro_rules! module_name_to_str {
    () => {
        fn module_name_to_str (c_str : & CStr) -> & str { c_str . to_str () . unwrap_or_else (| e | { bug ! ("Encountered non-utf8 LLVM module name `{}`: {}" , c_str . to_string_lossy () , e) }) }
    };
}

module_name_to_str!();