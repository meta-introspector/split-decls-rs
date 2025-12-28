macro_rules! SelfProfileAfterPassCallback {
    () => {
        pub (crate) type SelfProfileAfterPassCallback = unsafe extern "C" fn (* mut c_void) ;
    };
}

SelfProfileAfterPassCallback!();