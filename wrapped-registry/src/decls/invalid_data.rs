macro_rules! deps {
    () => {
        WIN32_ERROR!();
    };
}

macro_rules! invalid_data {
    () => {
        deps!();
        fn invalid_data () -> Error { Error :: from_hresult (WIN32_ERROR (ERROR_INVALID_DATA) . to_hresult ()) }
    };
}

invalid_data!();