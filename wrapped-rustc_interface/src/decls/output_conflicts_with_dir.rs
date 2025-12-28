macro_rules! output_conflicts_with_dir {
    () => {
        fn output_conflicts_with_dir (output_paths : & [PathBuf]) -> Option < & PathBuf > { output_paths . iter () . find (| output_path | output_path . is_dir ()) }
    };
}

output_conflicts_with_dir!();