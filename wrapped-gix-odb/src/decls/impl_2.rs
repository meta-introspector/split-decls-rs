macro_rules! deps {
    () => {
        RefreshMode!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl RefreshMode { # [doc = " Set this refresh mode to never refresh."] pub fn never (& mut self) { * self = RefreshMode :: Never ; } }
    };
}

impl_2!();