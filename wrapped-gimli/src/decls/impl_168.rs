macro_rules! deps {
    () => {
        EhFrameHdr!();
        Reader!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < R : Reader > From < R > for EhFrameHdr < R > { fn from (section : R) -> Self { EhFrameHdr (section) } }
    };
}

impl_168!();