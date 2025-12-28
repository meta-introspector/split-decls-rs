macro_rules! deps {
    () => {
        Error!();
        ServerError!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl From < parser :: Error > for ServerError { fn from (e : parser :: Error) -> Self { Self { message : e . to_string () , source : None , locations : e . positions () . collect () , path : Vec :: new () , extensions : None , } } }
    };
}

impl_34!()