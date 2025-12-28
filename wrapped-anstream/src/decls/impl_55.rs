macro_rules! deps {
    () => {
        RawStream!();
        Buffer!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        # [allow (deprecated)] impl RawStream for crate :: Buffer { }
    };
}

impl_55!()