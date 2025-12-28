macro_rules! deps {
    () => {
        Repository!();
        Object!();
        ObjectDetached!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl ObjectDetached { # [doc = " Infuse this owned object with `repo` access."] pub fn attach (self , repo : & crate :: Repository) -> Object < '_ > { Object { id : self . id , kind : self . kind , data : self . data , repo , } } }
    };
}

impl_242!();