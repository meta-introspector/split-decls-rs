macro_rules! deps {
    () => {
        Key!();
        Level!();
        Id!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl Index < Level > for Key { type Output = Id ; fn index (& self , index : Level) -> & Self :: Output { self . get (index) . expect ("key index in bound") } }
    };
}

impl_153!()