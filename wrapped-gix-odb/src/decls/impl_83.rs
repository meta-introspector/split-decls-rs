macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl PartialEq < Self > for Either { fn eq (& self , other : & Self) -> bool { self . path () . eq (other . path ()) } }
    };
}

impl_83!();