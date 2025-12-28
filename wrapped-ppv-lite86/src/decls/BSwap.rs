macro_rules! BSwap {
    () => {
        pub trait BSwap { fn bswap (self) -> Self ; }
    };
}

BSwap!()