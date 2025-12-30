// Generated macro for impl_321 (impl)
macro_rules! Depcrate_value_deimpl_321 {
() => {
// Module: crate::value::de
// Provides: {"impl_321"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for KeyClassifier { type Value = KeyClass ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a string key") } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : de :: Error , { match s { # [cfg (feature = "arbitrary_precision")] crate :: number :: TOKEN => Ok (KeyClass :: Number) , # [cfg (feature = "raw_value")] crate :: raw :: TOKEN => Ok (KeyClass :: RawValue) , _ => Ok (KeyClass :: Map (s . to_owned ())) , } } # [cfg (any (feature = "std" , feature = "alloc"))] fn visit_string < E > (self , s : String) -> Result < Self :: Value , E > where E : de :: Error , { match s . as_str () { # [cfg (feature = "arbitrary_precision")] crate :: number :: TOKEN => Ok (KeyClass :: Number) , # [cfg (feature = "raw_value")] crate :: raw :: TOKEN => Ok (KeyClass :: RawValue) , _ => Ok (KeyClass :: Map (s)) , } } }
};
}
