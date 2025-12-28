macro_rules! deps {
    () => {
        PrivateMarker!();
    };
}

macro_rules! private_decl {
    () => {
        deps!();
        macro_rules ! private_decl { () => { # [doc = " This trait is private; this method exists to make it"] # [doc = " impossible to implement outside the crate."] # [doc (hidden)] fn __rayon_private__ (& self) -> crate :: private :: PrivateMarker ; } ; }
    };
}

private_decl!();