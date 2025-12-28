macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_649 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < & '_ String > for Id { fn from (name : & '_ String) -> Self { Self (name . into ()) } }
    };
}

impl_649!()