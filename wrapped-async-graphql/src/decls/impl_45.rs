macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        # [cfg (not (feature = "custom-error-conversion"))] impl < T : Display + Send + Sync + 'static > From < T > for Error { fn from (e : T) -> Self { Self { message : e . to_string () , source : Some (Arc :: new (e)) , extensions : None , } } }
    };
}

impl_45!()