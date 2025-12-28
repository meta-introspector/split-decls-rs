macro_rules! build_dir_ignored_path_patterns {
    () => {
        # [doc = " The paths to ignore when [`CargoPathExt::assert_build_dir_layout`] is called"] fn build_dir_ignored_path_patterns () -> Vec < String > { vec ! ["[..].dSYM/[..]" , "[..].pdb" ,] . into_iter () . map (ToString :: to_string) . collect () }
    };
}

build_dir_ignored_path_patterns!();