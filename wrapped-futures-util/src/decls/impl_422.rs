macro_rules! deps {
    () => {
        FnOnce1!();
        NextIfEqFn!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        impl < T , Item > FnOnce1 < & Item > for NextIfEqFn < '_ , T , Item > where T : ? Sized , Item : PartialEq < T > , { type Output = bool ; fn call_once (self , next : & Item) -> Self :: Output { next == self . expected } }
    };
}

impl_422!()