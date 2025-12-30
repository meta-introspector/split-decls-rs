// Generated macro for impl_73 (impl)
macro_rules! Depcrate_defsimpl_73 {
() => {
// Module: crate::defs
// Provides: {"impl_73"}
// Dependencies: {}
impl TryFrom < Definition > for GenericDef { type Error = () ; fn try_from (def : Definition) -> Result < Self , Self :: Error > { match def { Definition :: Function (it) => Ok (it . into ()) , Definition :: Adt (it) => Ok (it . into ()) , Definition :: Trait (it) => Ok (it . into ()) , Definition :: TraitAlias (it) => Ok (it . into ()) , Definition :: TypeAlias (it) => Ok (it . into ()) , Definition :: SelfType (it) => Ok (it . into ()) , Definition :: Const (it) => Ok (it . into ()) , _ => Err (()) , } } }
};
}
