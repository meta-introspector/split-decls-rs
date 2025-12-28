macro_rules! deps {
    () => {
        Parse!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl From < http :: uri :: InvalidUri > for Parse { fn from (_ : http :: uri :: InvalidUri) -> Parse { Parse :: Uri } }
    };
}

impl_118!()