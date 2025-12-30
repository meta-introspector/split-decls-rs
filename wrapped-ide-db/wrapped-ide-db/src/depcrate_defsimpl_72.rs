// Generated macro for impl_72 (impl)
macro_rules! Depcrate_defsimpl_72 {
() => {
// Module: crate::defs
// Provides: {"impl_72"}
// Dependencies: {}
impl TryFrom < Definition > for GenericDef { type Error = () ; fn try_from (def : Definition) -> Result < Self , Self :: Error > { match def { Definition :: Function (it) => Ok (it . into ()) , Definition :: Adt (it) => Ok (it . into ()) , Definition :: Trait (it) => Ok (it . into ()) , Definition :: TypeAlias (it) => Ok (it . into ()) , Definition :: SelfType (it) => Ok (it . into ()) , Definition :: Const (it) => Ok (it . into ()) , _ => Err (()) , } } }
};
}
