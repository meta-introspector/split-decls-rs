// Generated macro for impl_272 (impl)
macro_rules! Depcrate_format_formattingimpl_272 {
() => {
// Module: crate::format::formatting
// Provides: {"impl_272"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , I : Iterator < Item = B > + Clone , B : Borrow < Item < 'a > > > Display for DelayedFormat < I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut result = String :: new () ; self . write_to (& mut result) ? ; f . pad (& result) } }
};
}
