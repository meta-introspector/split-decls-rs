// Generated macro for impl_253 (impl)
macro_rules! Depcrate_attributes_proc_macro_attrsimpl_253 {
() => {
// Module: crate::attributes::proc_macro_attrs
// Provides: {"impl_253"}
// Dependencies: {}
impl < S : Stage > SingleAttributeParser < S > for ProcMacroDeriveParser { const PATH : & [Symbol] = & [sym :: proc_macro_derive] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = PROC_MACRO_ALLOWED_TARGETS ; const TEMPLATE : AttributeTemplate = template ! (List : & ["TraitName" , "TraitName, attributes(name1, name2, ...)"] , "https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros") ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let (trait_name , helper_attrs) = parse_derive_like (cx , args , true) ? ; Some (AttributeKind :: ProcMacroDerive { trait_name : trait_name . expect ("Trait name is mandatory, so it is present") , helper_attrs , span : cx . attr_span , }) } }
};
}
