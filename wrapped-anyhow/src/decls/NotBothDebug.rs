macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! NotBothDebug {
    () => {
        deps!();
        # [doc (hidden)] pub trait NotBothDebug { fn __dispatch_ensure (self , msg : & 'static str) -> Error ; }
    };
}

NotBothDebug!();