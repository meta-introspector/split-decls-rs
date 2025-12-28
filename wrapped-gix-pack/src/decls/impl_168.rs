macro_rules! deps {
    () => {
        Count!();
        Location!();
        PackLocation!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl Count { # [doc = " Create a new instance from the given `oid` and its corresponding location."] pub fn from_data (oid : impl Into < ObjectId > , location : Option < crate :: data :: entry :: Location >) -> Self { Count { id : oid . into () , entry_pack_location : PackLocation :: LookedUp (location) , } } }
    };
}

impl_168!();