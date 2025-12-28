macro_rules! deps {
    () => {
        KeyType!();
    };
}

macro_rules! impl_540 {
    () => {
        deps!();
        impl PartialEq < str > for KeyType { fn eq (& self , rhs : & str) -> bool { match self { KeyType :: Long (l) => l == rhs , _ => false , } } }
    };
}

impl_540!()