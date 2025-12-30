// Generated macro for impl_224 (impl)
macro_rules! Depcrateimpl_224 {
() => {
// Module: crate
// Provides: {"impl_224"}
// Dependencies: {}
impl TryFrom < Value > for i64 { type Error = Value ; fn try_from (val : Value) -> Result < Self , Self :: Error > { match val { Value :: Integer (n) => match n . as_i64 () { Some (i) => Ok (i) , None => Err (val) , } , v => Err (v) , } } }
};
}
