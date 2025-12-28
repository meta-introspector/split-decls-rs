macro_rules! deps {
    () => {
        Token!();
        Interner!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T > Index < Token > for Interner < T > { type Output = T ; fn index (& self , index : Token) -> & Self :: Output { & self . tokens [index . 0 as usize] } }
    };
}

impl_17!()