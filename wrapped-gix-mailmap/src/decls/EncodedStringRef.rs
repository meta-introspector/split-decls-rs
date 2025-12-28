macro_rules! EncodedStringRef {
    () => {
        # [cfg_attr (test , derive (Debug))] # [derive (Clone , Copy)] pub enum EncodedStringRef < 'a > { Utf8 (& 'a str) , Unknown (& 'a BStr) , }
    };
}

EncodedStringRef!()