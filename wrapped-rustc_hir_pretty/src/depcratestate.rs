// Generated macro for State (struct)
macro_rules! DepcrateState {
() => {
// Module: crate
// Provides: {"State"}
// Dependencies: {}
pub struct State < 'a > { pub s : pp :: Printer , comments : Option < Comments < 'a > > , attrs : & 'a dyn Fn (HirId) -> & 'a [hir :: Attribute] , ann : & 'a (dyn PpAnn + 'a) , }
};
}
