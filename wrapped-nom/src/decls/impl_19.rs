macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < I : fmt :: Debug + fmt :: Display > std :: error :: Error for Error < I > { }
    };
}

impl_19!();