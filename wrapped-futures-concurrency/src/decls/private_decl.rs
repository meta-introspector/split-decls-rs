macro_rules! deps {
    () => {
        PrivateMarker!();
    };
}

macro_rules! private_decl {
    () => {
        deps!();
        # [doc (hidden)] # [macro_export] macro_rules ! private_decl { () => { # [doc = " This trait is private; this method exists to make it"] # [doc = " impossible to implement outside the crate."] # [doc (hidden)] fn __futures_concurrency_private__ (& self) -> $ crate :: private :: PrivateMarker ; } ; }
    };
}

private_decl!()