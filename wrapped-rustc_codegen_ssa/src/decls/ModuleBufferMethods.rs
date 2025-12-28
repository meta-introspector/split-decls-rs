macro_rules! ModuleBufferMethods {
    () => {
        pub trait ModuleBufferMethods : Send + Sync { fn data (& self) -> & [u8] ; }
    };
}

ModuleBufferMethods!();