// Generated macro for JustOnceFnAttributeExtractor (struct)
macro_rules! Depcrate_parse_just_onceJustOnceFnAttributeExtractor {
() => {
// Module: crate::parse::just_once
// Provides: {"JustOnceFnAttributeExtractor"}
// Dependencies: {}
# [doc = " Simple struct used to visit function attributes and extract attributes that match"] # [doc = " the `name`: Only one attribute is allowed for arguments."] pub struct JustOnceFnAttributeExtractor < 'a , B = () > where B : AttrBuilder < ItemFn > , { name : & 'a str , inner : Result < Option < B :: Out > , Vec < syn :: Error > > , _phantom : PhantomData < B > , }
};
}
