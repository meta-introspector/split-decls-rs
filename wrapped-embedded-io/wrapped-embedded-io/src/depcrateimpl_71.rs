// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl < E : core :: error :: Error + 'static > core :: error :: Error for ReadExactError < E > { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: UnexpectedEof => None , Self :: Other (error) => Some (error) , } } }
};
}
