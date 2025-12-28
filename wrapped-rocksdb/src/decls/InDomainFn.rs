macro_rules! InDomainFn {
    () => {
        pub type InDomainFn = fn (& [u8]) -> bool ;
    };
}

InDomainFn!();