// Generated macro for impl_34 (impl)
macro_rules! Depcrate_commit_message_bodyimpl_34 {
() => {
// Module: crate::commit::message::body
// Provides: {"impl_34"}
// Dependencies: {}
# [doc = " Convenience methods"] impl TrailerRef < '_ > { # [doc = " Check if this trailer is a `Signed-off-by` trailer (case-insensitive)."] pub fn is_signed_off_by (& self) -> bool { self . token . eq_ignore_ascii_case (b"Signed-off-by") } # [doc = " Check if this trailer is a `Co-authored-by` trailer (case-insensitive)."] pub fn is_co_authored_by (& self) -> bool { self . token . eq_ignore_ascii_case (b"Co-authored-by") } # [doc = " Check if this trailer is an `Acked-by` trailer (case-insensitive)."] pub fn is_acked_by (& self) -> bool { self . token . eq_ignore_ascii_case (b"Acked-by") } # [doc = " Check if this trailer is a `Reviewed-by` trailer (case-insensitive)."] pub fn is_reviewed_by (& self) -> bool { self . token . eq_ignore_ascii_case (b"Reviewed-by") } # [doc = " Check if this trailer is a `Tested-by` trailer (case-insensitive)."] pub fn is_tested_by (& self) -> bool { self . token . eq_ignore_ascii_case (b"Tested-by") } # [doc = " Check if this trailer represents any kind of authorship or attribution"] # [doc = " (`Signed-off-by`, `Co-authored-by`, etc.)."] pub fn is_attribution (& self) -> bool { self . is_signed_off_by () || self . is_co_authored_by () || self . is_acked_by () || self . is_reviewed_by () || self . is_tested_by () } }
};
}
