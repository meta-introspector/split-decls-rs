macro_rules! rotr_32 {
    () => {
        macro_rules ! rotr_32 { ($ name : ident , $ i : expr) => { # [inline (always)] fn $ name (self) -> Self { Self :: new (unsafe { _mm_or_si128 (_mm_srli_epi32 (self . x , $ i as i32) , _mm_slli_epi32 (self . x , 32 - $ i as i32) ,) }) } } ; }
    };
}

rotr_32!();