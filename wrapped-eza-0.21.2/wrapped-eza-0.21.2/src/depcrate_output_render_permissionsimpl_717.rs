// Generated macro for impl_717 (impl)
macro_rules! Depcrate_output_render_permissionsimpl_717 {
() => {
// Module: crate::output::render::permissions
// Provides: {"impl_717"}
// Dependencies: {}
impl f :: Permissions { fn user_execute_bit < C : Colours > (& self , colours : & C , is_regular_file : bool ,) -> ANSIString < 'static > { # [rustfmt :: skip] return match (self . user_execute , self . setuid , is_regular_file) { (false , false , _) => colours . dash () . paint ("-") , (true , false , false) => colours . user_execute_other () . paint ("x") , (true , false , true) => colours . user_execute_file () . paint ("x") , (false , true , _) => colours . special_other () . paint ("S") , (true , true , false) => colours . special_other () . paint ("s") , (true , true , true) => colours . special_user_file () . paint ("s") , } ; } fn group_execute_bit < C : Colours > (& self , colours : & C) -> ANSIString < 'static > { # [rustfmt :: skip] return match (self . group_execute , self . setgid) { (false , false) => colours . dash () . paint ("-") , (true , false) => colours . group_execute () . paint ("x") , (false , true) => colours . special_other () . paint ("S") , (true , true) => colours . special_other () . paint ("s") , } ; } fn other_execute_bit < C : Colours > (& self , colours : & C) -> ANSIString < 'static > { # [rustfmt :: skip] return match (self . other_execute , self . sticky) { (false , false) => colours . dash () . paint ("-") , (true , false) => colours . other_execute () . paint ("x") , (false , true) => colours . special_other () . paint ("T") , (true , true) => colours . special_other () . paint ("t") , } ; } }
};
}
