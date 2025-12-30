// Generated macro for AddressLookupTable (struct)
macro_rules! Depcrate_stateAddressLookupTable {
() => {
// Module: crate::state
// Provides: {"AddressLookupTable"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] # [derive (Debug , PartialEq , Eq , Clone)] pub struct AddressLookupTable < 'a > { pub meta : LookupTableMeta , pub addresses : Cow < 'a , [Pubkey] > , }
};
}
