macro_rules! Utf8Range {
    () => {
        # [doc = " A single inclusive range of UTF-8 bytes."] # [derive (Clone , Copy , Eq , PartialEq , PartialOrd , Ord)] pub struct Utf8Range { # [doc = " Start of byte range (inclusive)."] pub start : u8 , # [doc = " End of byte range (inclusive)."] pub end : u8 , }
    };
}

Utf8Range!()