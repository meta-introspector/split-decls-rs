macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! impl_727 {
    () => {
        deps!();
        impl PartialEq for Signature < '_ > { fn eq (& self , other : & Self) -> bool { self . when () == other . when () && self . email_bytes () == other . email_bytes () && self . name_bytes () == other . name_bytes () } }
    };
}

impl_727!()