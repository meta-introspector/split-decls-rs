macro_rules! deps {
    () => {
        KeyType!();
    };
}

macro_rules! impl_542 {
    () => {
        deps!();
        impl PartialEq < char > for KeyType { fn eq (& self , rhs : & char) -> bool { match self { KeyType :: Short (c) => c == rhs , _ => false , } } }
    };
}

impl_542!();