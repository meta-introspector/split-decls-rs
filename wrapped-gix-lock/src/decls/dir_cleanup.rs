macro_rules! dir_cleanup {
    () => {
        fn dir_cleanup (boundary : Option < PathBuf >) -> (ContainingDirectory , AutoRemove) { match boundary { None => (ContainingDirectory :: Exists , AutoRemove :: Tempfile) , Some (boundary_directory) => (ContainingDirectory :: CreateAllRaceProof (Default :: default ()) , AutoRemove :: TempfileAndEmptyParentDirectoriesUntil { boundary_directory } ,) , } }
    };
}

dir_cleanup!()