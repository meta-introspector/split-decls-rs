macro_rules! deps {
    () => {
        PrivateMarker!();
    };
}

macro_rules! private_impl {
    () => {
        deps!();
        # [doc (hidden)] # [macro_export] macro_rules ! private_impl { () => { fn __futures_concurrency_private__ (& self) -> $ crate :: private :: PrivateMarker { $ crate :: private :: PrivateMarker } } ; }
    };
}

private_impl!()