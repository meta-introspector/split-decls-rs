macro_rules! TOKEN_HANDOFF {
    () => {
        pub (crate) const TOKEN_HANDOFF : UnparkToken = UnparkToken (1) ;
    };
}

TOKEN_HANDOFF!();