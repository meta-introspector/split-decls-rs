macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl PartialEq < Id > for Id { fn eq (& self , other : & Id) -> bool { use subtle :: ConstantTimeEq ; (self . header . as_bytes () . ct_eq (other . header . as_bytes ()) & self . identifier . as_bytes () . ct_eq (other . identifier . as_bytes ())) . into () } }
    };
}

impl_66!();