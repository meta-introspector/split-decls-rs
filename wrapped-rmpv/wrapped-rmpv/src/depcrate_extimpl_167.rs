// Generated macro for impl_167 (impl)
macro_rules! Depcrate_extimpl_167 {
() => {
// Module: crate::ext
// Provides: {"impl_167"}
// Dependencies: {}
impl ValueExt for ValueRef < '_ > { # [cold] fn unexpected (& self) -> Unexpected < '_ > { match * self { ValueRef :: Nil => Unexpected :: Unit , ValueRef :: Boolean (v) => Unexpected :: Bool (v) , ValueRef :: Integer (Integer { n }) => match n { IntPriv :: PosInt (v) => Unexpected :: Unsigned (v) , IntPriv :: NegInt (v) => Unexpected :: Signed (v) , } , ValueRef :: F32 (v) => Unexpected :: Float (f64 :: from (v)) , ValueRef :: F64 (v) => Unexpected :: Float (v) , ValueRef :: String (ref v) => match v . s { Ok (v) => Unexpected :: Str (v) , Err (ref v) => Unexpected :: Bytes (v . 0) , } , ValueRef :: Binary (v) => Unexpected :: Bytes (v) , ValueRef :: Array (..) => Unexpected :: Seq , ValueRef :: Map (..) => Unexpected :: Map , ValueRef :: Ext (..) => Unexpected :: Seq , } } }
};
}
