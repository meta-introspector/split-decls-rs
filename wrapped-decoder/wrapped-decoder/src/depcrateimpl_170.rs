// Generated macro for impl_170 (impl)
macro_rules! Depcrateimpl_170 {
() => {
// Module: crate
// Provides: {"impl_170"}
// Dependencies: {}
impl Tag { fn to_level (& self) -> Option < Level > { match self { Tag :: Trace => Some (Level :: Trace) , Tag :: Debug => Some (Level :: Debug) , Tag :: Info => Some (Level :: Info) , Tag :: Warn => Some (Level :: Warn) , Tag :: Error => Some (Level :: Error) , _ => None , } } }
};
}
