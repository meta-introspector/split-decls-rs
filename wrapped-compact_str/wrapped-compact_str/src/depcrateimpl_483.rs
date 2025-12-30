// Generated macro for impl_483 (impl)
macro_rules! Depcrateimpl_483 {
() => {
// Module: crate
// Provides: {"impl_483"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < CompactString > for Box < dyn std :: error :: Error + Send + Sync > { fn from (value : CompactString) -> Self { struct StringError (CompactString) ; impl std :: error :: Error for StringError { # [allow (deprecated)] fn description (& self) -> & str { & self . 0 } } impl fmt :: Display for StringError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } } impl fmt :: Debug for StringError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } } Box :: new (StringError (value)) } }
};
}
