macro_rules! FieldElement {
    () => {
        # [derive (Clone , Copy , PartialEq , Debug)] # [doc = " Element in the field Z_q."] # [doc = ""] # [doc = " NOTE(brycx): While for Kyber q = 3329 a field element would fit in u16, but Dilithium q = 8380417 which only fits in u32."] # [doc = " Thus, for possible future re-usability, we use 32-bit integer here."] pub struct FieldElement (pub (crate) u32) ;
    };
}

FieldElement!();