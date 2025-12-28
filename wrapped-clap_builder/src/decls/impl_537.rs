macro_rules! deps {
    () => {
        KeyType!();
    };
}

macro_rules! impl_537 {
    () => {
        deps!();
        impl KeyType { pub (crate) fn is_position (& self) -> bool { matches ! (self , KeyType :: Position (_)) } }
    };
}

impl_537!()