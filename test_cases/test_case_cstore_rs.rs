// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_session/src/cstore.rs
// Error: expected square brackets
// Problematic line: line 11

use rustc_abi::ExternAbi;
use rustc_data_structures::sync::{self, AppendOnlyIndexVec, FreezeLock};
use rustc_hir::attrs::{CfgEntry, NativeLibKind, PeImportNameType};
use rustc_hir::def_id::{
    CrateNum, DefId, LOCAL_CRATE, LocalDefId, StableCrateId, StableCrateIdMap,
};
use rustc_hir::definitions::{DefKey, DefPath, DefPathHash, Definitions};
