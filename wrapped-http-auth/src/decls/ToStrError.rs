macro_rules! ToStrError {
    () => {
        # [doc = " An error returned by [`HeaderValue::to_str`]."] pub struct ToStrError { _priv : () , }
    };
}

ToStrError!()