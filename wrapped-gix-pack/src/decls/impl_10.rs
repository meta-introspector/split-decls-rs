macro_rules! deps {
    () => {
        Bundle!();
        Error!();
        Outcome!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Outcome { # [doc = " Instantiate a bundle from the newly written index and data file that are represented by this `Outcome`"] pub fn to_bundle (& self) -> Option < Result < crate :: Bundle , crate :: bundle :: init :: Error > > { self . index_path . as_ref () . map (| path | crate :: Bundle :: at (path , self . object_hash)) } }
    };
}

impl_10!();