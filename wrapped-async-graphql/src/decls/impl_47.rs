macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        # [cfg (feature = "custom-error-conversion")] impl From < String > for Error { fn from (e : String) -> Self { Self { message : e , source : None , extensions : None , } } }
    };
}

impl_47!()