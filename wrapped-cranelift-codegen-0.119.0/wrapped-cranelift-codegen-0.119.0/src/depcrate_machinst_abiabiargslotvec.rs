// Generated macro for ABIArgSlotVec (type)
macro_rules! Depcrate_machinst_abiABIArgSlotVec {
() => {
// Module: crate::machinst::abi
// Provides: {"ABIArgSlotVec"}
// Dependencies: {}
# [doc = " A vector of `ABIArgSlot`s. Inline capacity for one element because basically"] # [doc = " 100% of values use one slot. Only `i128`s need multiple slots, and they are"] # [doc = " super rare (and never happen with Wasm)."] pub type ABIArgSlotVec = SmallVec < ABIArgSlot , 1 > ;
};
}
