// Generated macro for impl_251 (impl)
macro_rules! Depcrate_enc_encoder_normalimpl_251 {
() => {
// Module: crate::enc::encoder_normal
// Provides: {"impl_251"}
// Dependencies: {}
impl Optimum { const INFINITY_PRICE : u32 = 1 << 30 ; fn reset (& mut self) { self . price = Self :: INFINITY_PRICE ; } fn set1 (& mut self , new_price : u32 , opt_cur : usize , back : i32) { self . price = new_price ; self . opt_prev = opt_cur ; self . back_prev = back ; self . prev1_is_literal = false ; } fn set2 (& mut self , new_price : u32 , opt_cur : usize , back : i32) { self . price = new_price ; self . opt_prev = opt_cur + 1 ; self . back_prev = back ; self . prev1_is_literal = true ; self . has_prev2 = false ; } fn set3 (& mut self , new_price : u32 , opt_cur : usize , back2 : i32 , len2 : usize , back : i32) { self . price = new_price ; self . opt_prev = opt_cur + len2 + 1 ; self . back_prev = back ; self . prev1_is_literal = true ; self . has_prev2 = true ; self . opt_prev2 = opt_cur ; self . back_prev2 = back2 ; } }
};
}
