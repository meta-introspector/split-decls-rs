// Generated macro for FunctionCallAction (struct)
macro_rules! DepcrateFunctionCallAction {
() => {
// Module: crate
// Provides: {"FunctionCallAction"}
// Dependencies: {}
# [derive (BorshSerialize , BorshDeserialize , Debug , Clone , Eq , PartialEq , SerdeSerialize , SerdeDeserialize , Readable , Writable ,)] pub struct FunctionCallAction { method_name : String , args : Vec < u8 > , gas : Gas , deposit : Balance , }
};
}
