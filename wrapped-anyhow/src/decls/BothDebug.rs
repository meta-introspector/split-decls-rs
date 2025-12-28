macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! BothDebug {
    () => {
        deps!();
        # [doc (hidden)] pub trait BothDebug { fn __dispatch_ensure (self , msg : & 'static str) -> Error ; }
    };
}

BothDebug!()