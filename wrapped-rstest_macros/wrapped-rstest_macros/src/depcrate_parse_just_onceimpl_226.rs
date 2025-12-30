// Generated macro for impl_226 (impl)
macro_rules! Depcrate_parse_just_onceimpl_226 {
() => {
// Module: crate::parse::just_once
// Provides: {"impl_226"}
// Dependencies: {}
impl < 'a , B > JustOnceFnAttributeExtractor < 'a , B > where B : AttrBuilder < ItemFn > , { pub fn new (name : & 'a str) -> Self { Self { name , inner : Ok (Default :: default ()) , _phantom : PhantomData , } } pub fn take (self) -> Result < Option < B :: Out > , ErrorsVec > { self . inner . map_err (Into :: into) } }
};
}
