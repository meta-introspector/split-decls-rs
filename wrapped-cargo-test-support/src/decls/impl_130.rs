macro_rules! deps {
    () => {
        HttpServerHandle!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl Drop for HttpServerHandle { fn drop (& mut self) { self . stop () ; } }
    };
}

impl_130!();