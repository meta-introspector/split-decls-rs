macro_rules! output_contains_path {
    () => {
        fn output_contains_path (output_paths : & [PathBuf] , input_path : & Path) -> bool { let input_path = try_canonicalize (input_path) . ok () ; if input_path . is_none () { return false ; } output_paths . iter () . any (| output_path | try_canonicalize (output_path) . ok () == input_path) }
    };
}

output_contains_path!();