macro_rules! deps {
    () => {
        TargetLoadError!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl From < String > for TargetLoadError { fn from (value : String) -> Self { Self (value . into ()) } }
    };
}

impl_70!();