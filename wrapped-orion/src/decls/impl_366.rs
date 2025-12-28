macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        impl PartialEq for FieldElement { fn eq (& self , other : & Self) -> bool { use subtle :: ConstantTimeEq ; self . as_bytes () . ct_eq (& other . as_bytes ()) . into () } }
    };
}

impl_366!();