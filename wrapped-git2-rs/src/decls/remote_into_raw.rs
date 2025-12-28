macro_rules! deps {
    () => {
        Remote!();
    };
}

macro_rules! remote_into_raw {
    () => {
        deps!();
        pub fn remote_into_raw (remote : Remote < '_ >) -> * mut raw :: git_remote { let ret = remote . raw ; mem :: forget (remote) ; ret }
    };
}

remote_into_raw!()