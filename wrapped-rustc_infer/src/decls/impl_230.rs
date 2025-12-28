macro_rules! deps {
    () => {
        TyVidSubKey!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl ut :: UnifyKey for TyVidSubKey { type Value = () ; # [inline] fn index (& self) -> u32 { self . vid . as_u32 () } # [inline] fn from_index (i : u32) -> TyVidSubKey { TyVidSubKey { vid : ty :: TyVid :: from_u32 (i) } } fn tag () -> & 'static str { "TyVidSubKey" } }
    };
}

impl_230!();