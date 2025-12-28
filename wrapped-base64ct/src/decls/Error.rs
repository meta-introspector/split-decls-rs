macro_rules! deps {
    () => {
        InvalidEncodingError!();
        InvalidLengthError!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Generic error, union of [`InvalidLengthError`] and [`InvalidEncodingError`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum Error { # [doc = " Invalid encoding of provided Base64 string."] InvalidEncoding , # [doc = " Insufficient output buffer length."] InvalidLength , }
    };
}

Error!();