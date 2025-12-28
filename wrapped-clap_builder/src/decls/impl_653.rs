macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_653 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < Cow < 'static , str > > for Id { fn from (name : Cow < 'static , str >) -> Self { Self (name . into ()) } }
    };
}

impl_653!();