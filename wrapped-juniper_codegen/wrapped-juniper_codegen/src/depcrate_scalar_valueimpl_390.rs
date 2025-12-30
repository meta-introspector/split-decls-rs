// Generated macro for impl_390 (impl)
macro_rules! Depcrate_scalar_valueimpl_390 {
() => {
// Module: crate::scalar_value
// Provides: {"impl_390"}
// Dependencies: {}
impl TryFrom < syn :: Fields > for Field { type Error = syn :: Error ; fn try_from (value : syn :: Fields) -> Result < Self , Self :: Error > { match value { syn :: Fields :: Named (mut f) if f . named . len () == 1 => { Ok (Self :: Named (Box :: new (f . named . pop () . unwrap () . into_value ()))) } syn :: Fields :: Unnamed (f) if f . unnamed . len () == 1 => Ok (Self :: Unnamed) , _ => Err (ERR . custom_error (value . span () , "expected exactly 1 field")) , } } }
};
}
