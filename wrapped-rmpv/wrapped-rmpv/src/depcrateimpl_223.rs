// Generated macro for impl_223 (impl)
macro_rules! Depcrateimpl_223 {
() => {
// Module: crate
// Provides: {"impl_223"}
// Dependencies: {}
impl TryFrom < Value > for u64 { type Error = Value ; fn try_from (val : Value) -> Result < Self , Self :: Error > { match val { Value :: Integer (n) => match n . as_u64 () { Some (i) => Ok (i) , None => Err (val) , } , v => Err (v) , } } }
};
}
