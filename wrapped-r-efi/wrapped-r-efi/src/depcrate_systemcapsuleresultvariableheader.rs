// Generated macro for CapsuleResultVariableHeader (struct)
macro_rules! Depcrate_systemCapsuleResultVariableHeader {
() => {
// Module: crate::system
// Provides: {"CapsuleResultVariableHeader"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct CapsuleResultVariableHeader { pub variable_total_size : u32 , pub reserved : u32 , pub capsule_guid : crate :: base :: Guid , pub capsule_processed : Time , pub capsule_status : crate :: base :: Status , }
};
}
