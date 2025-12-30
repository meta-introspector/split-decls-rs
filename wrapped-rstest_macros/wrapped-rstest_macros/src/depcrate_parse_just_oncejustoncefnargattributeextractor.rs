// Generated macro for JustOnceFnArgAttributeExtractor (struct)
macro_rules! Depcrate_parse_just_onceJustOnceFnArgAttributeExtractor {
() => {
// Module: crate::parse::just_once
// Provides: {"JustOnceFnArgAttributeExtractor"}
// Dependencies: {}
# [doc = " Simple struct used to visit function argument attributes and extract attributes that match"] # [doc = " the `name`: Only one attribute is allowed for arguments."] pub struct JustOnceFnArgAttributeExtractor < 'a , B = () > where B : AttrBuilder < Pat > , { name : & 'a str , elements : Vec < B :: Out > , errors : Vec < syn :: Error > , _phantom : PhantomData < B > , }
};
}
