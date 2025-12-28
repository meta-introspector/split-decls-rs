macro_rules! VendoredLibrariesConfig {
    () => {
        pub enum VendoredLibrariesConfig < 'a > { Included { workspace_root : & 'a VfsPath } , Excluded , }
    };
}

VendoredLibrariesConfig!()