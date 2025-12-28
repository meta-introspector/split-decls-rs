macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'i > PartialEq for Position < 'i > { fn eq (& self , other : & Position < 'i >) -> bool { ptr :: eq (self . input , other . input) && self . pos == other . pos } }
    };
}

impl_119!()