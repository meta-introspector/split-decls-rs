macro_rules! deps {
    () => {
        Arg!();
        MKeyMap!();
        KeyType!();
    };
}

macro_rules! impl_544 {
    () => {
        deps!();
        impl Index < & '_ KeyType > for MKeyMap { type Output = Arg ; fn index (& self , key : & KeyType) -> & Self :: Output { self . get (key) . expect (INTERNAL_ERROR_MSG) } }
    };
}

impl_544!()