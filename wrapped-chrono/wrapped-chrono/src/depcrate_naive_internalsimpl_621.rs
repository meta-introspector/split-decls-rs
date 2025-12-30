// Generated macro for impl_621 (impl)
macro_rules! Depcrate_naive_internalsimpl_621 {
() => {
// Module: crate::naive::internals
// Provides: {"impl_621"}
// Dependencies: {}
impl fmt :: Debug for Mdf { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let Mdf (mdf) = * self ; write ! (f , "Mdf(({} << 9) | ({} << 4) | {:#04o} /*{:?}*/)" , mdf >> 9 , (mdf >> 4) & 0b1_1111 , mdf & 0b1111 , YearFlags ((mdf & 0b1111) as u8)) } }
};
}
