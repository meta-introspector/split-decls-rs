macro_rules! LitKind {
    () => {
        # [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub enum LitKind { Bool , Byte , Char , Integer , Float , Str , StrRaw (u8) , ByteStr , ByteStrRaw (u8) , CStr , CStrRaw (u8) , Err (ErrorGuaranteed) , }
    };
}

LitKind!();