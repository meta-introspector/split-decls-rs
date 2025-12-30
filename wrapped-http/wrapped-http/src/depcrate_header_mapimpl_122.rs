// Generated macro for impl_122 (impl)
macro_rules! Depcrate_header_mapimpl_122 {
() => {
// Module: crate::header::map
// Provides: {"impl_122"}
// Dependencies: {}
impl Danger { fn is_red (& self) -> bool { matches ! (* self , Danger :: Red (_)) } fn set_red (& mut self) { debug_assert ! (self . is_yellow ()) ; * self = Danger :: Red (RandomState :: new ()) ; } fn is_yellow (& self) -> bool { matches ! (* self , Danger :: Yellow) } fn set_yellow (& mut self) { if let Danger :: Green = * self { * self = Danger :: Yellow ; } } fn set_green (& mut self) { debug_assert ! (self . is_yellow ()) ; * self = Danger :: Green ; } }
};
}
