macro_rules! deps {
    () => {
        OsStr!();
        KeyType!();
    };
}

macro_rules! impl_541 {
    () => {
        deps!();
        impl PartialEq < OsStr > for KeyType { fn eq (& self , rhs : & OsStr) -> bool { match self { KeyType :: Long (l) => l == rhs , _ => false , } } }
    };
}

impl_541!()