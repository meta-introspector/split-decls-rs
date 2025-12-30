// Generated macro for impl_65 (impl)
macro_rules! Depcrate_typesimpl_65 {
() => {
// Module: crate::types
// Provides: {"impl_65"}
// Dependencies: {}
impl Directive { # [doc = " Attempt to convert this `Directive` into a `ConstDirective`."] # [must_use] pub fn into_const (self) -> Option < ConstDirective > { Some (ConstDirective { name : self . name , arguments : self . arguments . into_iter () . map (| (name , value) | { Some ((name , Positioned :: new (value . node . into_const () ? , value . pos))) }) . collect :: < Option < _ > > () ? , }) } # [doc = " Get the argument with the given name."] # [must_use] pub fn get_argument (& self , name : & str) -> Option < & Positioned < Value > > { self . arguments . iter () . find (| item | item . 0 . node == name) . map (| item | & item . 1) } }
};
}
