// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_metadata/src/creader.rs
// Error: expected square brackets
// Problematic line: line 27

use rustc_middle::ty::{TyCtxt, TyCtxtFeed};
use rustc_proc_macro::bridge::client::ProcMacro;
use rustc_session::Session;
use rustc_session::config::{
    CrateType, ExtendedTargetModifierInfo, ExternLocation, Externs, OptionsTargetModifiers,
    TargetModifier,
};
