macro_rules! deps {
    () => {
        MockItemStruct!();
        MockItemModule!();
    };
}

macro_rules! MockItem {
    () => {
        deps!();
        # [doc = " A Mock item"] pub (crate) enum MockItem { Module (MockItemModule) , Struct (MockItemStruct) }
    };
}

MockItem!()