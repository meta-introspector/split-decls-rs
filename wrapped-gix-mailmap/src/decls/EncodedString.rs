macro_rules! EncodedString {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq)] pub enum EncodedString { Utf8 (String) , Unknown (BString) , }
    };
}

EncodedString!()