// Generated macro for impl_84 (impl)
macro_rules! Depcrate_valueimpl_84 {
() => {
// Module: crate::value
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'sval > ValueSlice < 'sval > { fn new < 'a > (parts : & 'a [ValuePart < 'sval >]) -> & 'a ValueSlice < 'sval > { unsafe { mem :: transmute :: < & 'a [ValuePart < 'sval >] , & 'a ValueSlice < 'sval > > (parts) } } fn get (& self , i : usize) -> Option < & ValuePart < 'sval > > { self . 0 . get (i) } fn slice < 'a > (& 'a self , range : Range < usize >) -> & 'a ValueSlice < 'sval > { match self . 0 . get (range . clone ()) { Some (_) => () , None => { panic ! ("{:?} is out of range for {:?}" , range , & self . 0) ; } } unsafe { mem :: transmute :: < & 'a [ValuePart < 'sval >] , & 'a ValueSlice < 'sval > > (& self . 0 [range]) } } }
};
}
