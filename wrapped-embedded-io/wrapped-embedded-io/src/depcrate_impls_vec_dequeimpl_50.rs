// Generated macro for impl_50 (impl)
macro_rules! Depcrate_impls_vec_dequeimpl_50 {
() => {
// Module: crate::impls::vec_deque
// Provides: {"impl_50"}
// Dependencies: {}
# [doc = " Write is implemented for `VecDeque<u8>` by appending to the `VecDeque`, growing it as needed."] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl Write for VecDeque < u8 > { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { self . extend (buf) ; Ok (buf . len ()) } # [inline] fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { self . extend (buf) ; Ok (()) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
