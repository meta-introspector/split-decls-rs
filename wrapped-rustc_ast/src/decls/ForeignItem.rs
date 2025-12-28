macro_rules! deps {
    () => {
        ForeignItemKind!();
        Item!();
    };
}

macro_rules! ForeignItem {
    () => {
        deps!();
        pub type ForeignItem = Item < ForeignItemKind > ;
    };
}

ForeignItem!()