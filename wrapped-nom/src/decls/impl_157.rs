macro_rules! deps {
    () => {
        Error!();
        Err!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < E > Error for Err < E > where E : fmt :: Debug , { fn source (& self) -> Option < & (dyn Error + 'static) > { None } }
    };
}

impl_157!()