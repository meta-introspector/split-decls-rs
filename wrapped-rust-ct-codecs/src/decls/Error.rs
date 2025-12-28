macro_rules! deps {
    () => {
        Base64!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Error type for ct-codecs operations."] # [doc = ""] # [doc = " This enum represents the possible error conditions that can occur"] # [doc = " during encoding and decoding operations."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum Error { # [doc = " The provided output buffer would be too small to hold the result."] # [doc = ""] # [doc = " This error occurs when:"] # [doc = " - The output buffer passed to an encode/decode function is too small"] # [doc = " - A calculation would result in an integer overflow"] Overflow , # [doc = " The input isn't valid for the given encoding."] # [doc = ""] # [doc = " This error occurs when:"] # [doc = " - A Base64 string contains invalid characters"] # [doc = " - A Base64 string has invalid padding"] # [doc = " - A hex string contains non-hexadecimal characters"] # [doc = " - A hex string has an odd length"] InvalidInput , }
    };
}

Error!();