macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_648 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < String > for Id { fn from (name : String) -> Self { Self (name . into ()) } }
    };
}

impl_648!()