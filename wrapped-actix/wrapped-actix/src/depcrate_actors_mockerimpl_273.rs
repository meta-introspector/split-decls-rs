// Generated macro for impl_273 (impl)
macro_rules! Depcrate_actors_mockerimpl_273 {
() => {
// Module: crate::actors::mocker
// Provides: {"impl_273"}
// Dependencies: {}
impl < T : Unpin > Mocker < T > { # [allow (clippy :: type_complexity)] pub fn mock (mock : Box < dyn FnMut (Box < dyn Any > , & mut Context < Mocker < T > >) -> Box < dyn Any > > ,) -> Mocker < T > { Mocker :: < T > { phantom : PhantomData , mock , } } }
};
}
