// Generated macro for impl_70 (impl)
macro_rules! Depcrate_defsimpl_70 {
() => {
// Module: crate::defs
// Provides: {"impl_70"}
// Dependencies: {}
impl TryFrom < DefWithBody > for Definition { type Error = () ; fn try_from (def : DefWithBody) -> Result < Self , Self :: Error > { match def { DefWithBody :: Function (it) => Ok (it . into ()) , DefWithBody :: Static (it) => Ok (it . into ()) , DefWithBody :: Const (it) => Ok (it . into ()) , DefWithBody :: Variant (it) => Ok (it . into ()) , } } }
};
}
