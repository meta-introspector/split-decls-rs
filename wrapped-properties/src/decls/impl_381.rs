macro_rules! deps {
    () => {
        ScriptWithExt!();
        Script!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        impl AsULE for ScriptWithExt { type ULE = < u16 as AsULE > :: ULE ; # [inline] fn to_unaligned (self) -> Self :: ULE { Script (self . 0) . to_unaligned () } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { ScriptWithExt (Script :: from_unaligned (unaligned) . 0) } }
    };
}

impl_381!()