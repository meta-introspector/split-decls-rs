// Generated macro for impl_41 (impl)
macro_rules! Depcrate_legacyimpl_41 {
() => {
// Module: crate::legacy
// Provides: {"impl_41"}
// Dependencies: {}
impl Sanitize for Message { fn sanitize (& self) -> std :: result :: Result < () , SanitizeError > { if self . header . num_required_signatures as usize + self . header . num_readonly_unsigned_accounts as usize > self . account_keys . len () { return Err (SanitizeError :: IndexOutOfBounds) ; } if self . header . num_readonly_signed_accounts >= self . header . num_required_signatures { return Err (SanitizeError :: IndexOutOfBounds) ; } for ci in & self . instructions { if ci . program_id_index as usize >= self . account_keys . len () { return Err (SanitizeError :: IndexOutOfBounds) ; } if ci . program_id_index == 0 { return Err (SanitizeError :: IndexOutOfBounds) ; } for ai in & ci . accounts { if * ai as usize >= self . account_keys . len () { return Err (SanitizeError :: IndexOutOfBounds) ; } } } self . account_keys . sanitize () ? ; self . recent_blockhash . sanitize () ? ; self . instructions . sanitize () ? ; Ok (()) } }
};
}
