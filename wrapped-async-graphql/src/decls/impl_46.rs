macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        # [cfg (feature = "custom-error-conversion")] impl From < & 'static str > for Error { fn from (e : & 'static str) -> Self { Self { message : e . to_string () , source : None , extensions : None , } } }
    };
}

impl_46!()