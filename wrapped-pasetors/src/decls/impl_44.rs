macro_rules! deps {
    () => {
        AsymmetricPublicKey!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < V : Version > PartialEq < AsymmetricPublicKey < V > > for AsymmetricPublicKey < V > { fn eq (& self , other : & AsymmetricPublicKey < V >) -> bool { use subtle :: ConstantTimeEq ; self . as_bytes () . ct_eq (other . as_bytes ()) . into () } }
    };
}

impl_44!()