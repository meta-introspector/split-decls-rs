macro_rules! impl_into {
    () => {
        macro_rules ! impl_into { ($ storage : ident , $ array : ty , $ name : ident) => { impl From <$ storage > for $ array { # [inline (always)] fn from (vec : $ storage) -> Self { unsafe { vec .$ name } } } } ; }
    };
}

impl_into!()