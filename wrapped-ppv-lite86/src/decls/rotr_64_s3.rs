macro_rules! rotr_64_s3 {
    () => {
        macro_rules ! rotr_64_s3 { ($ name : ident , $ k0 : expr , $ k1 : expr) => { # [inline (always)] fn $ name (self) -> Self { Self :: new (unsafe { _mm_shuffle_epi8 (self . x , _mm_set_epi64x ($ k0 , $ k1)) }) } } ; }
    };
}

rotr_64_s3!();