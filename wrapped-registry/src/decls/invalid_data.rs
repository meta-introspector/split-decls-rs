macro_rules! invalid_data {
    () => {
        fn invalid_data () -> Error { Error :: from_hresult (WIN32_ERROR (ERROR_INVALID_DATA) . to_hresult ()) }
    };
}

invalid_data!()