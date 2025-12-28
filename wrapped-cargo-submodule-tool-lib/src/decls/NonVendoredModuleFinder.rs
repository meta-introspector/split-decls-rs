macro_rules! NonVendoredModuleFinder {
    () => {
        pub trait NonVendoredModuleFinder { fn find_and_count_non_vendored (& self , tree_file_path : & Path , project_root : & Path ,) -> Result < HashMap < String , u32 > > ; }
    };
}

NonVendoredModuleFinder!()