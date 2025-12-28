macro_rules! trim {
    () => {
        fn trim (mut wide : & [u16]) -> & [u16] { while wide . last () == Some (& 0) { wide = & wide [.. wide . len () - 1] ; } wide }
    };
}

trim!()