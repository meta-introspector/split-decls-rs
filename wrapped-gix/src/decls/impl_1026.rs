macro_rules! deps {
    () => {
        Change!();
        Item!();
    };
}

macro_rules! impl_1026 {
    () => {
        deps!();
        impl From < gix_diff :: index :: Change > for Item { fn from (value : gix_diff :: index :: Change) -> Self { Item :: TreeIndex (value) } }
    };
}

impl_1026!();