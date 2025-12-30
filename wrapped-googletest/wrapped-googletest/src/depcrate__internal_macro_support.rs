// Generated macro for __internal_macro_support (module)
macro_rules! Depcrate__internal_macro_support {
() => {
// Module: crate
// Provides: {"__internal_macro_support"}
// Dependencies: {}
# [doc (hidden)] pub mod __internal_macro_support { use crate :: fixtures :: Fixture ; pub struct FixtureTearDownOnDrop < T : Fixture > { fixture : Option < T > , } impl < T : Fixture > FixtureTearDownOnDrop < T > { pub fn new (fixture : T) -> Self { Self { fixture : Some (fixture) } } pub fn tear_down (mut self) -> crate :: Result < () > { if let Some (fixture) = self . fixture . take () { fixture . tear_down () } else { Ok (()) } } } impl < T : Fixture > AsRef < T > for FixtureTearDownOnDrop < T > { fn as_ref (& self) -> & T { self . fixture . as_ref () . unwrap () } } impl < T : Fixture > AsMut < T > for FixtureTearDownOnDrop < T > { fn as_mut (& mut self) -> & mut T { self . fixture . as_mut () . unwrap () } } impl < T : Fixture > Drop for FixtureTearDownOnDrop < T > { fn drop (& mut self) { if let Some (fixture) = self . fixture . take () { fixture . tear_down () . unwrap () ; } } } }
};
}
