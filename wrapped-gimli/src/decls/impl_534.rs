macro_rules! deps {
    () => {
        DebugPubNames!();
        DebugLookup!();
        Reader!();
    };
}

macro_rules! impl_534 {
    () => {
        deps!();
        impl < R : Reader > From < R > for DebugPubNames < R > { fn from (debug_pubnames_section : R) -> Self { DebugPubNames (DebugLookup :: from (debug_pubnames_section)) } }
    };
}

impl_534!();