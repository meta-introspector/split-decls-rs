macro_rules! deps {
    () => {
        DebugPubTypes!();
        Reader!();
        DebugLookup!();
    };
}

macro_rules! impl_546 {
    () => {
        deps!();
        impl < R : Reader > From < R > for DebugPubTypes < R > { fn from (debug_pubtypes_section : R) -> Self { DebugPubTypes (DebugLookup :: from (debug_pubtypes_section)) } }
    };
}

impl_546!()