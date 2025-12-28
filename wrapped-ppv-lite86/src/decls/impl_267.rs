macro_rules! impl_267 {
    () => {
        impl From < vec256_storage > for [u64 ; 4] { # [inline (always)] fn from (q : vec256_storage) -> Self { let [a , b] : [u64 ; 2] = q . v128 [0] . into () ; let [c , d] : [u64 ; 2] = q . v128 [1] . into () ; [a , b , c , d] } }
    };
}

impl_267!()