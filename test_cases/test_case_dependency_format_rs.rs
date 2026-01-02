// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_metadata/src/dependency_format.rs
// Error: expected square brackets
// Problematic line: line 67

use tracing::info;

use crate::creader::CStore;
use crate::errors::{
    BadPanicStrategy, CrateDepMultiple, IncompatiblePanicInDropStrategy, LibRequired,
    NonStaticCrateDep, RequiredPanicStrategy, RlibRequired, RustcDriverHelp, RustcLibRequired,
    TwoPanicRuntimes,
