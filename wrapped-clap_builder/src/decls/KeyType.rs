macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! KeyType {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Hash , Clone)] pub (crate) enum KeyType { Short (char) , Long (OsStr) , Position (usize) , }
    };
}

KeyType!();