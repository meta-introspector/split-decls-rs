macro_rules! adjust_ip {
    () => {
        fn adjust_ip (a : * mut c_void) -> * mut c_void { if a . is_null () { a } else { (a as usize - 1) as * mut c_void } }
    };
}

adjust_ip!()