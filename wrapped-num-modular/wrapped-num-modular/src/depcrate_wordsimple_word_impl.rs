// Generated macro for simple_word_impl (macro)
macro_rules! Depcrate_wordsimple_word_impl {
() => {
// Module: crate::word
// Provides: {"simple_word_impl"}
// Dependencies: {}
macro_rules ! simple_word_impl { ($ S : ty , $ D : ident) => { pub type Word = $ S ; pub type DoubleWord = $ D ; pub use super ::$ D as DoubleWordModule ; # [inline (always)] pub const fn ones (n : u32) -> Word { if n == 0 { 0 } else { Word :: MAX >> (Word :: BITS - n) } } # [inline (always)] pub const fn extend (word : Word) -> DoubleWord { word as DoubleWord } # [inline (always)] pub const fn low (dw : DoubleWord) -> Word { dw as Word } # [inline (always)] pub const fn high (dw : DoubleWord) -> Word { (dw >> Word :: BITS) as Word } # [inline (always)] pub const fn split (dw : DoubleWord) -> (Word , Word) { (low (dw) , high (dw)) } # [inline (always)] pub const fn merge (low : Word , high : Word) -> DoubleWord { extend (low) | extend (high) << Word :: BITS } # [doc = " Widening multiplication"] # [inline (always)] pub const fn wmul (a : Word , b : Word) -> DoubleWord { extend (a) * extend (b) } # [doc = " Widening squaring"] # [inline (always)] pub const fn wsqr (a : Word) -> DoubleWord { extend (a) * extend (a) } # [doc = " Narrowing remainder"] pub const fn nrem (n : DoubleWord , d : Word) -> Word { (n % d as DoubleWord) as _ } } ; }
};
}
