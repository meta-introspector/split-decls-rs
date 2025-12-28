macro_rules! deps {
    () => {
        Hash128!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl FromStableHash for Hash128 { type Hash = StableHasherHash ; # [inline] fn from (StableHasherHash ([_0 , _1]) : Self :: Hash) -> Self { Self { inner : u128 :: from (_0) | (u128 :: from (_1) << 64) } } }
    };
}

impl_9!()