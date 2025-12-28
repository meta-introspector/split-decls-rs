macro_rules! deps {
    () => {
        RegionVidKey!();
        RegionVariableValue!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl < 'tcx > UnifyKey for RegionVidKey < 'tcx > { type Value = RegionVariableValue < 'tcx > ; # [inline] fn index (& self) -> u32 { self . vid . as_u32 () } # [inline] fn from_index (i : u32) -> Self { RegionVidKey :: from (ty :: RegionVid :: from_u32 (i)) } fn tag () -> & 'static str { "RegionVidKey" } }
    };
}

impl_236!();