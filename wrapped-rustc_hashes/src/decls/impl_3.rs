macro_rules! deps {
    () => {
        Hash64!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl FromStableHash for Hash64 { type Hash = StableHasherHash ; # [inline] fn from (StableHasherHash ([_0 , __1]) : Self :: Hash) -> Self { Self { inner : _0 } } }
    };
}

impl_3!()