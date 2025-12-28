macro_rules! deps {
    () => {
        CapacityError!();
        String!();
    };
}

macro_rules! FromUtf16Error {
    () => {
        deps!();
        # [doc = " A possible error value when converting a [`String`] from a UTF-16 byte slice."] # [doc = ""] # [doc = " This type is the error type for the [`from_utf16`] method on [`String`]."] # [doc = ""] # [doc = " [`from_utf16`]: String::from_utf16"] # [derive (Debug)] pub enum FromUtf16Error { # [doc = " The capacity of the `String` is too small for the given operation."] Capacity (CapacityError) , # [doc = " Error decoding UTF-16."] DecodeUtf16 (DecodeUtf16Error) , }
    };
}

FromUtf16Error!();