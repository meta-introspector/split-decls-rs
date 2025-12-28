macro_rules! deps {
    () => {
        Macro!();
        ItemInNs!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl From < Macro > for ItemInNs { fn from (it : Macro) -> Self { Self :: Macros (it) } }
    };
}

impl_320!();