macro_rules! deps {
    () => {
        Parse!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl From < http :: status :: InvalidStatusCode > for Parse { fn from (_ : http :: status :: InvalidStatusCode) -> Parse { Parse :: Status } }
    };
}

impl_117!();