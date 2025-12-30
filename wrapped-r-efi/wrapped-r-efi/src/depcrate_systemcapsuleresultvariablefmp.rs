// Generated macro for CapsuleResultVariableFMP (struct)
macro_rules! Depcrate_systemCapsuleResultVariableFMP {
() => {
// Module: crate::system
// Provides: {"CapsuleResultVariableFMP"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct CapsuleResultVariableFMP < const N : usize = 0 > { pub version : u16 , pub payload_index : u8 , pub update_image_index : u8 , pub update_image_type_id : crate :: base :: Guid , pub capsule_file_name_and_target : [crate :: base :: Char16 ; N] , }
};
}
