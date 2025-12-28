macro_rules! deps {
    () => {
        SmallIndex!();
    };
}

macro_rules! impl_761 {
    () => {
        deps!();
        impl From < u8 > for SmallIndex { fn from (index : u8) -> SmallIndex { SmallIndex :: new_unchecked (usize :: from (index)) } }
    };
}

impl_761!()