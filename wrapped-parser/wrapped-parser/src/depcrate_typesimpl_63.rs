// Generated macro for impl_63 (impl)
macro_rules! Depcrate_typesimpl_63 {
() => {
// Module: crate::types
// Provides: {"impl_63"}
// Dependencies: {}
impl ConstDirective { # [doc = " Convert this `ConstDirective` into a `Directive`."] # [must_use] pub fn into_directive (self) -> Directive { Directive { name : self . name , arguments : self . arguments . into_iter () . map (| (name , value) | (name , value . map (ConstValue :: into_value))) . collect () , } } # [doc = " Get the argument with the given name."] # [must_use] pub fn get_argument (& self , name : & str) -> Option < & Positioned < ConstValue > > { self . arguments . iter () . find (| item | item . 0 . node == name) . map (| item | & item . 1) } }
};
}
