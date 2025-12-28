macro_rules! deps {
    () => {
        StringId!();
    };
}

macro_rules! Name {
    () => {
        deps!();
        # [doc = " A section or symbol name."] # [derive (Debug , Clone , Copy)] pub enum Name { # [doc = " An inline name."] Short ([u8 ; 8]) , # [doc = " An id of a string table entry."] Long (StringId) , }
    };
}

Name!()