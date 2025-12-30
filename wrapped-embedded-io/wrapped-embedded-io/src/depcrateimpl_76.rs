// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
impl < E : core :: error :: Error + 'static > core :: error :: Error for WriteFmtError < E > { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: FmtError => None , Self :: Other (error) => Some (error) , } } }
};
}
