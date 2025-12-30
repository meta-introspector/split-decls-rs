// Generated macro for impl_225 (impl)
macro_rules! Depcrateimpl_225 {
() => {
// Module: crate
// Provides: {"impl_225"}
// Dependencies: {}
impl TryFrom < Value > for f64 { type Error = Value ; fn try_from (val : Value) -> Result < Self , Self :: Error > { match val { Value :: Integer (n) => match n . as_f64 () { Some (i) => Ok (i) , None => Err (val) , } , Value :: F32 (n) => Ok (From :: from (n)) , Value :: F64 (n) => Ok (n) , v => Err (v) , } } }
};
}
