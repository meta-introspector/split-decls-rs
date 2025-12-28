macro_rules! deps {
    () => {
        AttrItem!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl AttrItem { pub fn is_valid_for_outer_style (& self) -> bool { self . path == sym :: cfg_attr || self . path == sym :: cfg || self . path == sym :: forbid || self . path == sym :: warn || self . path == sym :: allow || self . path == sym :: deny } }
    };
}

impl_195!();