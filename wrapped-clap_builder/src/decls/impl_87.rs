macro_rules! deps {
    () => {
        Arg!();
        Id!();
        Command!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Index < & '_ Id > for Command { type Output = Arg ; fn index (& self , key : & Id) -> & Self :: Output { self . find (key) . expect (INTERNAL_ERROR_MSG) } }
    };
}

impl_87!();