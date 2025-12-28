macro_rules! CharStr {
    () => {
        # [doc = " A single-char string."] # [derive (Copy , Clone , Debug)] pub struct CharStr { buf : [u8 ; 4] , len : u32 , }
    };
}

CharStr!()