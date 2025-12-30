// Generated macro for revoke_pending_activation (function)
macro_rules! Depcrate_instructionrevoke_pending_activation {
() => {
// Module: crate::instruction
// Provides: {"revoke_pending_activation"}
// Dependencies: {}
# [doc = " Creates a 'RevokePendingActivation' instruction."] # [cfg (feature = "bincode")] pub fn revoke_pending_activation (feature_id : & Pubkey) -> Instruction { let accounts = vec ! [AccountMeta :: new (* feature_id , true) , AccountMeta :: new (incinerator :: id () , false) , AccountMeta :: new_readonly (system_program :: id () , false) ,] ; Instruction { program_id : crate :: id () , accounts , data : vec ! [0] , } }
};
}
