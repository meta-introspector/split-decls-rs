macro_rules! FormError {
    () => {
        # [doc = " An error from \"form add\" operations."] # [doc = ""] # [doc = " THis structure wraps a `CURLFORMcode`."] # [derive (Clone , PartialEq)] pub struct FormError { code : curl_sys :: CURLFORMcode , }
    };
}

FormError!();