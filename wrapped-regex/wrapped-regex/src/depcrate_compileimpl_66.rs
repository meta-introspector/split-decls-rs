// Generated macro for impl_66 (impl)
macro_rules! Depcrate_compileimpl_66 {
() => {
// Module: crate::compile
// Provides: {"impl_66"}
// Dependencies: {}
impl ByteClassSet { fn new () -> Self { ByteClassSet ([false ; 256]) } fn set_range (& mut self , start : u8 , end : u8) { debug_assert ! (start <= end) ; if start > 0 { self . 0 [start as usize - 1] = true ; } self . 0 [end as usize] = true ; } fn set_word_boundary (& mut self) { let iswb = is_word_byte ; let mut b1 : u16 = 0 ; let mut b2 : u16 ; while b1 <= 255 { b2 = b1 + 1 ; while b2 <= 255 && iswb (b1 as u8) == iswb (b2 as u8) { b2 += 1 ; } self . set_range (b1 as u8 , (b2 - 1) as u8) ; b1 = b2 ; } } fn byte_classes (& self) -> Vec < u8 > { let mut byte_classes = vec ! [0 ; 256] ; let mut class = 0u8 ; for i in 0 .. 256 { byte_classes [i] = class ; if self . 0 [i] { class = class . checked_add (1) . unwrap () ; } } byte_classes } }
};
}
