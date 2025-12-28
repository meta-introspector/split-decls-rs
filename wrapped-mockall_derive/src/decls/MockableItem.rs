macro_rules! deps {
    () => {
        MockableStruct!();
        MockableModule!();
    };
}

macro_rules! MockableItem {
    () => {
        deps!();
        # [doc = " An item that's ready to be mocked."] # [doc = ""] # [doc = " It should be functionally identical or near-identical to the original item,"] # [doc = " but with minor alterations that make it suitable for mocking, such as"] # [doc = " altered lifetimes."] pub (crate) enum MockableItem { Module (MockableModule) , Struct (MockableStruct) }
    };
}

MockableItem!();