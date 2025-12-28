macro_rules! deps {
    () => {
        Parse!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl From < http :: method :: InvalidMethod > for Parse { fn from (_ : http :: method :: InvalidMethod) -> Parse { Parse :: Method } }
    };
}

impl_116!();