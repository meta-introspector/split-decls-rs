// Generated macro for error (module)
macro_rules! Depcrate_config_snapshot_credential_helperserror {
() => {
// Module: crate::config::snapshot::credential_helpers
// Provides: {"error"}
// Dependencies: {}
mod error { use crate :: bstr :: BString ; # [doc = " The error returned by [`Snapshot::credential_helpers()`][super::Snapshot::credential_helpers()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not parse 'useHttpPath' key in section {section}")] InvalidUseHttpPath { section : BString , source : gix_config :: value :: Error , } , # [error ("core.askpass could not be read")] CoreAskpass (# [from] gix_config :: path :: interpolate :: Error) , # [error (transparent)] BooleanConfig (# [from] crate :: config :: boolean :: Error) , } }
};
}
