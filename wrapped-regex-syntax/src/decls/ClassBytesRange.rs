macro_rules! ClassBytesRange {
    () => {
        # [doc = " A single range of characters represented by arbitrary bytes."] # [doc = ""] # [doc = " The range is closed. That is, the start and end of the range are included"] # [doc = " in the range."] # [derive (Clone , Copy , Default , Eq , PartialEq , PartialOrd , Ord)] pub struct ClassBytesRange { start : u8 , end : u8 , }
    };
}

ClassBytesRange!();