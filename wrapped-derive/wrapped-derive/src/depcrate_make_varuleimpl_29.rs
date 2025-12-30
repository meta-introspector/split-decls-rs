// Generated macro for impl_29 (impl)
macro_rules! Depcrate_make_varuleimpl_29 {
() => {
// Module: crate::make_varule
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a > UnsizedField < 'a > { fn new (field : & 'a Field , index : usize , custom_varule_ident : Option < Ident > ,) -> Result < Self , String > { Ok (UnsizedField { kind : UnsizedFieldKind :: new (& field . ty , custom_varule_ident) ? , field : FieldInfo :: new_for_field (field , index) , }) } # [doc = " Call `<Self as EncodeAsVarULE<V>>::#method(self.accessor #additional_args)` after adjusting"] # [doc = " Self and self.accessor to be the right types"] fn encode_func (& self , method : TokenStream2 , additional_args : TokenStream2) -> TokenStream2 { let encodeas_trait = quote ! (zerovec :: ule :: EncodeAsVarULE) ; let (encodeable_ty , encodeable) = self . encodeable_tokens () ; let varule_ty = self . kind . varule_ty () ; quote ! (<# encodeable_ty as # encodeas_trait <# varule_ty >>::# method (# encodeable , # additional_args)) } # [doc = " Returns (encodeable_ty, encodeable)"] fn encodeable_tokens (& self) -> (TokenStream2 , TokenStream2) { let accessor = self . field . accessor . clone () ; let value = quote ! (self .# accessor) ; let encodeable = self . kind . encodeable_value (value) ; let encodeable_ty = self . kind . encodeable_ty () ; (encodeable_ty , encodeable) } }
};
}
