// Generated macro for FunctionCallPermission (struct)
macro_rules! DepcrateFunctionCallPermission {
() => {
// Module: crate
// Provides: {"FunctionCallPermission"}
// Dependencies: {}
# [derive (BorshSerialize , BorshDeserialize , Debug , Clone , Eq , PartialEq , SerdeSerialize , SerdeDeserialize , Readable , Writable ,)] pub struct FunctionCallPermission { allowance : Option < Balance > , receiver_id : AccountId , method_names : Vec < String > , }
};
}
