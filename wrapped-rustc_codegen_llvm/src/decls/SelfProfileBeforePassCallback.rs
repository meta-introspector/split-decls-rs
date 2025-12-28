macro_rules! SelfProfileBeforePassCallback {
    () => {
        pub (crate) type SelfProfileBeforePassCallback = unsafe extern "C" fn (* mut c_void , * const c_char , * const c_char) ;
    };
}

SelfProfileBeforePassCallback!()