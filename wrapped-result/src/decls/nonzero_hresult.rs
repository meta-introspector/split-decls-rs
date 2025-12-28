macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! nonzero_hresult {
    () => {
        deps!();
        fn nonzero_hresult (hr : HRESULT) -> NonZeroI32 { if let Some (nz) = NonZeroI32 :: new (hr . 0) { nz } else { S_EMPTY_ERROR } }
    };
}

nonzero_hresult!()