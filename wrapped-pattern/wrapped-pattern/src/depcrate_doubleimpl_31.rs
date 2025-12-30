// Generated macro for impl_31 (impl)
macro_rules! Depcrate_doubleimpl_31 {
() => {
// Module: crate::double
// Provides: {"impl_31"}
// Dependencies: {}
impl DoublePlaceholderInfo { pub fn from_char (ch : char) -> Self { Self { key : if ((ch as usize) & 0x1) == 0 { DoublePlaceholderKey :: Place0 } else { DoublePlaceholderKey :: Place1 } , offset : (ch as usize) >> 1 , } } # [cfg (feature = "alloc")] pub fn try_to_char (self) -> Result < char , Error > { let usize_val = match self . key { DoublePlaceholderKey :: Place0 => 0 , DoublePlaceholderKey :: Place1 => 1 , } | (self . offset << 1) ; u32 :: try_from (usize_val) . ok () . and_then (| x | char :: try_from (x) . ok ()) . ok_or (Error :: InvalidPattern) } # [doc = " Creates a PlaceholderInfo for an empty Place0"] pub fn no_place0 () -> Self { Self { key : DoublePlaceholderKey :: Place0 , offset : 0 , } } # [doc = " Changes Place0 to Place1 and vice-versa"] pub fn swap (self) -> Self { Self { key : match self . key { DoublePlaceholderKey :: Place0 => DoublePlaceholderKey :: Place1 , DoublePlaceholderKey :: Place1 => DoublePlaceholderKey :: Place0 , } , offset : self . offset , } } # [doc = " Sets the offset to 0 (ignored placeholder), retaining the key"] # [cfg (feature = "alloc")] pub fn clear (self) -> Self { Self { key : self . key , offset : 0 , } } }
};
}
