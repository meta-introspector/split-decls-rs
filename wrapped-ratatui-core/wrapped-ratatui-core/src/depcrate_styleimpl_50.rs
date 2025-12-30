// Generated macro for impl_50 (impl)
macro_rules! Depcrate_styleimpl_50 {
() => {
// Module: crate::style
// Provides: {"impl_50"}
// Dependencies: {}
# [doc = " Implement the `Debug` trait for `Modifier` manually."] # [doc = ""] # [doc = " This will avoid printing the empty modifier as 'Borders(0x0)' and instead print it as 'NONE'."] impl fmt :: Debug for Modifier { # [doc = " Format the modifier as `NONE` if the modifier is empty or as a list of flags separated by"] # [doc = " `|` otherwise."] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . is_empty () { return write ! (f , "NONE") ; } write ! (f , "{}" , self . 0) } }
};
}
