// Generated macro for impl_166 (impl)
macro_rules! Depcrate_extimpl_166 {
() => {
// Module: crate::ext
// Provides: {"impl_166"}
// Dependencies: {}
impl ValueExt for Value { # [cold] fn unexpected (& self) -> Unexpected < '_ > { match * self { Self :: Nil => Unexpected :: Unit , Self :: Boolean (v) => Unexpected :: Bool (v) , Self :: Integer (Integer { n }) => match n { IntPriv :: PosInt (v) => Unexpected :: Unsigned (v) , IntPriv :: NegInt (v) => Unexpected :: Signed (v) , } , Self :: F32 (v) => Unexpected :: Float (f64 :: from (v)) , Self :: F64 (v) => Unexpected :: Float (v) , Self :: String (ref v) => match v . s { Ok (ref v) => Unexpected :: Str (v) , Err (ref v) => Unexpected :: Bytes (& v . 0 [..]) , } , Self :: Binary (ref v) => Unexpected :: Bytes (v) , Self :: Array (..) => Unexpected :: Seq , Self :: Map (..) => Unexpected :: Map , Self :: Ext (..) => Unexpected :: Seq , } } }
};
}
