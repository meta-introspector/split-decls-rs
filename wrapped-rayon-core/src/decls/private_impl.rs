macro_rules! deps {
    () => {
        PrivateMarker!();
    };
}

macro_rules! private_impl {
    () => {
        deps!();
        macro_rules ! private_impl { () => { fn __rayon_private__ (& self) -> crate :: private :: PrivateMarker { crate :: private :: PrivateMarker } } ; }
    };
}

private_impl!();