// Generated macro for impl_75 (impl)
macro_rules! Depcrate_abi_exampleimpl_75 {
() => {
// Module: crate::abi_example
// Provides: {"impl_75"}
// Dependencies: {}
impl < T : Default + Serialize > TypeErasedExample < T > for Placeholder { default fn type_erased_example () -> T { let original_type_name = type_name :: < T > () ; let normalized_type_name = normalize_type_name (original_type_name) ; if normalized_type_name . starts_with ("solana") { panic ! ("derive or implement AbiExample/AbiEnumVisitor for {original_type_name}") ; } else { panic ! ("new unrecognized type for ABI digest!: {original_type_name}") } } }
};
}
