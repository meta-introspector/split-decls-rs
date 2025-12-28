macro_rules! deps {
    () => {
        MockableStruct!();
        MockableItem!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl From < MockableStruct > for MockableItem { fn from (mock : MockableStruct) -> MockableItem { MockableItem :: Struct (mock) } }
    };
}

impl_85!();