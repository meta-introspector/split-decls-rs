macro_rules! deps {
    () => {
        HeaderSlice!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < H , T > HeaderSlice < H , [T] > { pub (crate) fn slice (& self) -> & [T] { & self . slice } }
    };
}

impl_147!();