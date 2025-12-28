macro_rules! FARPROC {
    () => {
        pub type FARPROC = Option < unsafe extern "system" fn () -> isize > ;
    };
}

FARPROC!()