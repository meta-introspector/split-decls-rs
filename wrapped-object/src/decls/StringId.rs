macro_rules! StringId {
    () => {
        # [doc = " An identifier for an entry in a string table."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct StringId (usize) ;
    };
}

StringId!();