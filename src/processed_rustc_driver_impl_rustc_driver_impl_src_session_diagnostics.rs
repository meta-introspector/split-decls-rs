// SRC: ../rust/compiler/rustc_driver_impl/src/session_diagnostics.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use std::error::Error;

use rustc_macros::{Diagnostic, Subdiagnostic};
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=CantEmitMIR | COMPLEXITY=2 | LINES=6 */

#[derive(Diagnostic)]
#[diag(driver_impl_cant_emit_mir)]
pub struct CantEmitMIR {
    pub error: std::io::Error,
}
/* AST_META: AST_ID=3 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Diagnostic)]
#[diag(driver_impl_rlink_unable_to_read)]
pub(crate) struct RlinkUnableToRead {
    pub err: std::io::Error,
}
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=15 */

#[derive(Diagnostic)]
#[diag(driver_impl_rlink_wrong_file_type)]
pub(crate) struct RLinkWrongFileType;

#[derive(Diagnostic)]
#[diag(driver_impl_rlink_empty_version_number)]
pub(crate) struct RLinkEmptyVersionNumber;

#[derive(Diagnostic)]
#[diag(driver_impl_rlink_encoding_version_mismatch)]
pub(crate) struct RLinkEncodingVersionMismatch {
    pub version_array: String,
    pub rlink_version: u32,
}
/* AST_META: AST_ID=5 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[derive(Diagnostic)]
#[diag(driver_impl_rlink_rustc_version_mismatch)]
pub(crate) struct RLinkRustcVersionMismatch<'a> {
    pub rustc_version: String,
    pub current_version: &'a str,
}
/* AST_META: AST_ID=6 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(driver_impl_rlink_no_a_file)]
pub(crate) struct RlinkNotAFile;

#[derive(Diagnostic)]
#[diag(driver_impl_rlink_corrupt_file)]
pub(crate) struct RlinkCorruptFile<'a> {
    pub file: &'a std::path::Path,
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(driver_impl_ice)]
pub(crate) struct Ice;

#[derive(Diagnostic)]
#[diag(driver_impl_ice_bug_report)]
pub(crate) struct IceBugReport<'a> {
    pub bug_report_url: &'a str,
}
/* AST_META: AST_ID=8 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=15 */

#[derive(Diagnostic)]
#[diag(driver_impl_ice_bug_report_update_note)]
pub(crate) struct UpdateNightlyNote;

#[derive(Diagnostic)]
#[diag(driver_impl_ice_bug_report_internal_feature)]
pub(crate) struct IceBugReportInternalFeature;

#[derive(Diagnostic)]
#[diag(driver_impl_ice_version)]
pub(crate) struct IceVersion<'a> {
    pub version: &'a str,
    pub triple: &'a str,
}
/* AST_META: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Diagnostic)]
#[diag(driver_impl_ice_path)]
pub(crate) struct IcePath {
    pub path: std::path::PathBuf,
}
/* AST_META: AST_ID=10 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

#[derive(Diagnostic)]
#[diag(driver_impl_ice_path_error)]
pub(crate) struct IcePathError {
    pub path: std::path::PathBuf,
    pub error: String,
    #[subdiagnostic]
    pub env_var: Option<IcePathErrorEnv>,
}
/* AST_META: AST_ID=11 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Subdiagnostic)]
#[note(driver_impl_ice_path_error_env)]
pub(crate) struct IcePathErrorEnv {
    pub env_var: std::path::PathBuf,
}
/* AST_META: AST_ID=12 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Diagnostic)]
#[diag(driver_impl_ice_flags)]
pub(crate) struct IceFlags {
    pub flags: String,
}
/* AST_META: AST_ID=13 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

#[derive(Diagnostic)]
#[diag(driver_impl_ice_exclude_cargo_defaults)]
pub(crate) struct IceExcludeCargoDefaults;

#[derive(Diagnostic)]
#[diag(driver_impl_unstable_feature_usage)]
pub(crate) struct UnstableFeatureUsage {
    pub error: Box<dyn Error>,
}