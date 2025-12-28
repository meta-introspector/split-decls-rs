macro_rules! deps {
    () => {
        KeyType!();
    };
}

macro_rules! impl_538 {
    () => {
        deps!();
        impl PartialEq < usize > for KeyType { fn eq (& self , rhs : & usize) -> bool { match self { KeyType :: Position (x) => x == rhs , _ => false , } } }
    };
}

impl_538!();