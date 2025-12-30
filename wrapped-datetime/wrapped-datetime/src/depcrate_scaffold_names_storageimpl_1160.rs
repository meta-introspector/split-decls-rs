// Generated macro for impl_1160 (impl)
macro_rules! Depcrate_scaffold_names_storageimpl_1160 {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"impl_1160"}
// Dependencies: {}
impl < M : DynamicDataMarker , Variables > MaybePayload < M , Variables > for DataPayloadWithVariables < M , Variables > where Variables : PartialEq + Copy + MaybeAsErrorField , { # [inline] fn new_empty () -> Self { Self { inner : OptionalNames :: None , } } fn load_put < P > (& mut self , provider : & P , req : DataRequest , variables : Variables ,) -> Result < Result < DataResponseMetadata , DataError > , MaybePayloadError > where P : BoundDataProvider < M > + ? Sized , Self : Sized , { let arg_variables = variables ; match & self . inner { OptionalNames :: SingleLength { variables , .. } if arg_variables == * variables => { return Ok (Ok (Default :: default ())) ; } OptionalNames :: SingleLength { variables , .. } => { let loaded_field = match variables . maybe_as_error_field () { Some (x) => x , None => { debug_assert ! (false , "all non-unit variables implement this trait") ; use crate :: provider :: fields :: * ; ErrorField (Field { symbol : FieldSymbol :: Era , length : FieldLength :: Six , }) } } ; return Err (MaybePayloadError :: ConflictingField (loaded_field)) ; } OptionalNames :: None => () , } ; match provider . load_bound (req) { Ok (response) => { self . inner = OptionalNames :: SingleLength { payload : response . payload , variables : arg_variables , } ; Ok (Ok (response . metadata)) } Err (e) => Ok (Err (e)) , } } # [inline] fn get (& self) -> DataPayloadWithVariablesBorrowed < '_ , M , Variables > { DataPayloadWithVariablesBorrowed { inner : self . inner . as_borrowed () , } } }
};
}
