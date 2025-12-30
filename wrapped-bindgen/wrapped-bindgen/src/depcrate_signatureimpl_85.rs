// Generated macro for impl_85 (impl)
macro_rules! Depcrate_signatureimpl_85 {
() => {
// Module: crate::signature
// Provides: {"impl_85"}
// Dependencies: {}
impl Signature { pub fn size (& self) -> usize { self . params . iter () . fold (0 , | sum , param | sum + std :: cmp :: max (4 , param . size ())) } pub fn types (& self) -> impl Iterator < Item = & Type > + '_ { std :: iter :: once (& self . return_type) . chain (self . params . iter () . map (| param | & param . ty)) . map (| ty | ty . decay ()) } pub fn is_retval (& self) -> bool { if let Some (param) = self . params . last () { if param . def . has_attribute ("RetValAttribute") { return true ; } } if let Some (param) = self . params . last () { if param . is_retval () { return self . params [.. self . params . len () - 1] . iter () . all (| param | param . is_input ()) ; } } false } }
};
}
