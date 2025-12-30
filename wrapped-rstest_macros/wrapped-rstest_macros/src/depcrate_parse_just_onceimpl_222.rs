// Generated macro for impl_222 (impl)
macro_rules! Depcrate_parse_just_onceimpl_222 {
() => {
// Module: crate::parse::just_once
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'a , B > JustOnceFnArgAttributeExtractor < 'a , B > where B : AttrBuilder < Pat > , { pub fn new (name : & 'a str) -> Self { Self { name , elements : Default :: default () , errors : Default :: default () , _phantom : PhantomData , } } pub fn take (self) -> Result < Vec < B :: Out > , ErrorsVec > { if self . errors . is_empty () { Ok (self . elements) } else { Err (self . errors . into ()) } } }
};
}
