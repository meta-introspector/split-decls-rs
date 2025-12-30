// Generated macro for impl_29 (impl)
macro_rules! Depcrate_mimeimpl_29 {
() => {
// Module: crate::mime
// Provides: {"impl_29"}
// Dependencies: {}
impl Mime { # [doc = " Construct a new [`Mime`] with the given `type_` and `subtype` and an"] # [doc = " empty parameter list."] pub fn new (type_ : & str , subtype : & str) -> Self { Self { type_ : type_ . into () , subtype : subtype . into () , parameters : vec ! [] , } } # [doc = " Return true if this [`Mime`] matches a given type and subtype, regardless"] # [doc = " of what parameters it has."] pub fn matches (& self , type_ : & str , subtype : & str) -> bool { self . type_ == type_ && self . subtype == subtype } pub fn get_parameter < P > (& self , name : & P) -> Option < & str > where P : ? Sized + PartialEq < str > , { self . parameters . iter () . find (| & (n , _) | name == & * * n) . map (| (_ , v) | & * * v) } }
};
}
