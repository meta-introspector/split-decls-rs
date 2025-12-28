macro_rules! deps {
    () => {
        SymmetricKey!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < V : Version > PartialEq < SymmetricKey < V > > for SymmetricKey < V > { fn eq (& self , other : & SymmetricKey < V >) -> bool { use subtle :: ConstantTimeEq ; self . as_bytes () . ct_eq (other . as_bytes ()) . into () } }
    };
}

impl_36!()