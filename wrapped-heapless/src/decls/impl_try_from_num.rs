macro_rules! deps {
    () => {
        String!();
        LenType!();
    };
}

macro_rules! impl_try_from_num {
    () => {
        deps!();
        macro_rules ! impl_try_from_num { ($ num : ty , $ size : expr) => { impl < LenT : LenType , const N : usize > core :: convert :: TryFrom <$ num > for String < N , LenT > { type Error = () ; fn try_from (s : $ num) -> Result < Self , Self :: Error > { let mut new = String :: new () ; write ! (& mut new , "{}" , s) . map_err (| _ | ()) ?; Ok (new) } } } ; }
    };
}

impl_try_from_num!();