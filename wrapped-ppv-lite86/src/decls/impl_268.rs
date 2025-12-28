macro_rules! impl_268 {
    () => {
        impl From < [u64 ; 4] > for vec256_storage { # [inline (always)] fn from ([a , b , c , d] : [u64 ; 4]) -> Self { Self { v128 : [[a , b] . into () , [c , d] . into ()] , } } }
    };
}

impl_268!()