macro_rules! deps {
    () => {
        Tree!();
        Repository!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        # [doc = " Initialization"] impl < 'repo > Tree < 'repo > { # [doc = " Obtain a tree instance by handing in all components that it is made up of."] pub fn from_data (id : impl Into < ObjectId > , data : Vec < u8 > , repo : & 'repo crate :: Repository) -> Self { Tree { id : id . into () , data , repo , } } }
    };
}

impl_185!();