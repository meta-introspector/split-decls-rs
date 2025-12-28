macro_rules! ReaderOffsetId {
    () => {
        # [doc = " An identifier for an offset within a section reader."] # [doc = ""] # [doc = " This is used for error reporting. The meaning of this value is specific to"] # [doc = " each reader implementation. The values should be chosen to be unique amongst"] # [doc = " all readers. If values are not unique then errors may point to the wrong reader."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct ReaderOffsetId (pub u64) ;
    };
}

ReaderOffsetId!();