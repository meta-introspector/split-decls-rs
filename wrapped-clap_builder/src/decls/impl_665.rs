macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_665 {
    () => {
        deps!();
        impl PartialEq < String > for Id { # [inline] fn eq (& self , other : & String) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
    };
}

impl_665!()