macro_rules! deps {
    () => {
        DebugLine!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        impl < R > From < R > for DebugLine < R > { fn from (debug_line_section : R) -> Self { DebugLine { debug_line_section } } }
    };
}

impl_404!()