macro_rules! deps {
    () => {
        RefSpec!();
        RefSpecRef!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [doc = " Conversion"] impl RefSpecRef < '_ > { # [doc = " Convert this ref into a standalone, owned copy."] pub fn to_owned (& self) -> RefSpec { RefSpec { mode : self . mode , op : self . op , src : self . src . map (ToOwned :: to_owned) , dst : self . dst . map (ToOwned :: to_owned) , } } }
    };
}

impl_14!();