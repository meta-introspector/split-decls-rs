macro_rules! deps {
    () => {
        DebugAbbrev!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl < R > From < R > for DebugAbbrev < R > { fn from (debug_abbrev_section : R) -> Self { DebugAbbrev { debug_abbrev_section , } } }
    };
}

impl_337!()