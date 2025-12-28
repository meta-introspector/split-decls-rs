macro_rules! invalid_guid {
    () => {
        fn invalid_guid () -> Error { Error :: from_hresult (imp :: E_INVALIDARG) }
    };
}

invalid_guid!();