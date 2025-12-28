macro_rules! deps {
    () => {
        Warnings!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl Warnings { # [doc = " Panics if any warnings are present."] # [track_caller] pub fn unwrap (& self) { if ! self . is_empty () { panic ! ("{self}") ; } } }
    };
}

impl_330!();