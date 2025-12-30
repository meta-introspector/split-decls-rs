// Generated macro for flat_types (function)
macro_rules! Depcrate_abiflat_types {
() => {
// Module: crate::abi
// Provides: {"flat_types"}
// Dependencies: {}
# [doc = " Flatten types in a given type"] # [doc = ""] # [doc = " It is sometimes necessary to restrict the number of max parameters dynamically,"] # [doc = " for example during an async guest import call (flat params are limited to 4)"] fn flat_types (resolve : & Resolve , ty : & Type , max_params : Option < usize >) -> Option < Vec < WasmType > > { let max_params = max_params . unwrap_or (MAX_FLAT_PARAMS) ; let mut storage = iter :: repeat_n (WasmType :: I32 , max_params) . collect :: < Vec < _ > > () ; let mut flat = FlatTypes :: new (storage . as_mut_slice ()) ; resolve . push_flat (ty , & mut flat) . then_some (flat . to_vec ()) }
};
}
