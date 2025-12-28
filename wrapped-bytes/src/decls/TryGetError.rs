macro_rules! deps {
    () => {
        Buf!();
    };
}

macro_rules! TryGetError {
    () => {
        deps!();
        # [doc = " Error type for the `try_get_` methods of [`Buf`]."] # [doc = " Indicates that there were not enough remaining"] # [doc = " bytes in the buffer while attempting"] # [doc = " to get a value from a [`Buf`] with one"] # [doc = " of the `try_get_` methods."] # [derive (Debug , PartialEq , Eq)] pub struct TryGetError { # [doc = " The number of bytes necessary to get the value"] pub requested : usize , # [doc = " The number of bytes available in the buffer"] pub available : usize , }
    };
}

TryGetError!();