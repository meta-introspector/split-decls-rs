macro_rules! deps {
    () => {
        Parse!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl From < http :: uri :: InvalidUriParts > for Parse { fn from (_ : http :: uri :: InvalidUriParts) -> Parse { Parse :: Uri } }
    };
}

impl_119!();