macro_rules! deps {
    () => {
        MockItemModule!();
        MockItemStruct!();
    };
}

macro_rules! MockItem {
    () => {
        deps!();
        # [doc = " A Mock item"] pub (crate) enum MockItem { Module (MockItemModule) , Struct (MockItemStruct) }
    };
}

MockItem!();