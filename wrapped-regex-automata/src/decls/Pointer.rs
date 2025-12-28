macro_rules! Pointer {
    () => {
        pub (crate) trait Pointer { fn as_usize (self) -> usize ; }
    };
}

Pointer!();