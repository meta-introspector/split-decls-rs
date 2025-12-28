macro_rules! deps {
    () => {
        AddressSpace!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl AddressSpace { # [doc = " LLVM's `0` address space."] pub const ZERO : Self = AddressSpace (0) ; }
    };
}

impl_60!()