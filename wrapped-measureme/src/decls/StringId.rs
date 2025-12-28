macro_rules! StringId {
    () => {
        # [doc = " A `StringId` is used to identify a string in the `StringTable`. It is"] # [doc = " either a regular `StringId`, meaning that it contains the absolute address"] # [doc = " of a string within the string table data. Or it is \"virtual\", which means"] # [doc = " that the address it points to is resolved via the string table index data,"] # [doc = " that maps virtual `StringId`s to addresses."] # [derive (Clone , Copy , Eq , PartialEq , Debug , Hash)] # [repr (C)] pub struct StringId (u64) ;
    };
}

StringId!();