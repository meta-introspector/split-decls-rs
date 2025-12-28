macro_rules! ThinBufferMethods {
    () => {
        pub trait ThinBufferMethods : Send + Sync { fn data (& self) -> & [u8] ; }
    };
}

ThinBufferMethods!();