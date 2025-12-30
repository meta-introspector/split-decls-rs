// Generated macro for impl_1163 (impl)
macro_rules! Depcrate_scaffold_names_storageimpl_1163 {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"impl_1163"}
// Dependencies: {}
impl < Variables , Payload > OptionalNames < Variables , Payload > where Variables : Copy + PartialEq , Payload : Copy , { pub (crate) fn get_with_variables (& self , arg_variables : Variables) -> Option < Payload > { match self { Self :: None => None , Self :: SingleLength { variables , payload } if arg_variables == * variables => { Some (* payload) } _ => None , } } }
};
}
