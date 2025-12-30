// Generated macro for impl_11 (impl)
macro_rules! Depcrate_encodingimpl_11 {
() => {
// Module: crate::encoding
// Provides: {"impl_11"}
// Dependencies: {}
# [doc = " Formats this [`Encoding`] in a similar way that the `@encode` directive"] # [doc = " would ordinarily do."] # [doc = ""] # [doc = " You should not rely on the output of this to be stable across versions. It"] # [doc = " may change if found to be required to be compatible with existing"] # [doc = " Objective-C compilers."] impl fmt :: Display for Encoding { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { Helper :: new (self) . fmt (f , NestingLevel :: new ()) } }
};
}
