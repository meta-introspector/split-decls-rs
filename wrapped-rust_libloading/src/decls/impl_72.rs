macro_rules! deps {
    () => {
        DlError!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < & CStr > for DlError { fn from (value : & CStr) -> Self { Self (value . into ()) } }
    };
}

impl_72!();