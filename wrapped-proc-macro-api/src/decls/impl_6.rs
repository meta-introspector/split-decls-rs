macro_rules! deps {
    () => {
        MacroDylib!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl MacroDylib { # [doc = " Creates a new MacroDylib instance with the given path."] pub fn new (path : AbsPathBuf) -> MacroDylib { MacroDylib { path } } }
    };
}

impl_6!()