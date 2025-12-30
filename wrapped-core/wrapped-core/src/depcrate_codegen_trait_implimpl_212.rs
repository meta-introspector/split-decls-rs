// Generated macro for impl_212 (impl)
macro_rules! Depcrate_codegen_trait_implimpl_212 {
() => {
// Module: crate::codegen::trait_impl
// Provides: {"impl_212"}
// Dependencies: {}
impl < 'a > TraitImpl < 'a > { # [doc = " Get all declared type parameters."] pub fn declared_type_params (& self) -> IdentSet { self . generics . type_params () . map (| tp | tp . ident . clone ()) . collect () } # [doc = " Get the type parameters which are used by non-skipped, non-magic fields."] # [doc = " These type parameters will have a `FromMeta` bound applied to them in emitted"] # [doc = " code."] pub fn used_type_params (& self) -> IdentSet { self . type_params_matching (| f | ! f . skip , | v | ! v . skip) } fn type_params_matching < F , V > (& self , field_filter : F , variant_filter : V) -> IdentSet where F : Fn (& & Field < '_ >) -> bool , V : Fn (& & Variant < '_ >) -> bool , { let declared = self . declared_type_params () ; match self . data { Data :: Struct (ref v) => self . type_params_in_fields (v , & field_filter , & declared) , Data :: Enum (ref v) => { v . iter () . filter (variant_filter) . fold (Default :: default () , | mut state , variant | { state . extend (self . type_params_in_fields (& variant . data , & field_filter , & declared ,)) ; state }) } } } # [doc = " Get the type parameters of all fields in a set matching some filter"] fn type_params_in_fields < 'b , F > (& 'b self , fields : & 'b Fields < Field < 'a > > , field_filter : F , declared : & IdentSet ,) -> IdentSet where F : Fn (& & 'b Field < '_ >) -> bool , { fields . iter () . filter (field_filter) . collect_type_params_cloned (& Purpose :: BoundImpl . into () , declared) } }
};
}
