macro_rules! deps {
    () => {
        AddressSpace!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl AddressSpace { # [doc = " LLVM's `0` address space."] pub const ZERO : Self = AddressSpace (0) ; }
    };
}

impl_101!();