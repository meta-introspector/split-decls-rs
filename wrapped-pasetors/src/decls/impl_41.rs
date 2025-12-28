macro_rules! deps {
    () => {
        AsymmetricSecretKey!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < V : Version > PartialEq < AsymmetricSecretKey < V > > for AsymmetricSecretKey < V > { fn eq (& self , other : & AsymmetricSecretKey < V >) -> bool { use subtle :: ConstantTimeEq ; self . as_bytes () . ct_eq (other . as_bytes ()) . into () } }
    };
}

impl_41!()