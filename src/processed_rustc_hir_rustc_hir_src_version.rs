// SRC: ../rust/compiler/rustc_hir/src/version.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use std::borrow::Cow;
use std::fmt::{self, Display};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use std::sync::OnceLock;

use crate::rustc_error_messages::{DiagArgValue, IntoDiagArg};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use rustc_macros::{
    Decodable, Encodable, HashStable_Generic, PrintAttribute, current_rustc_version,
};
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=RustcVersion | COMPLEXITY=2 | LINES=10 */

use crate::attrs::PrintAttribute;

#[derive(Encodable, Decodable, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(HashStable_Generic, PrintAttribute)]
pub struct RustcVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=current_overridable | COMPLEXITY=11 | LINES=23 */

impl RustcVersion {
    pub const CURRENT: Self = current_rustc_version!();
    pub fn current_overridable() -> Self {
        *CURRENT_OVERRIDABLE.get_or_init(|| {
            if let Ok(override_var) = std::env::var("RUSTC_OVERRIDE_VERSION_STRING")
                && let Some(override_) = Self::parse_str(&override_var)
            {
                override_
            } else {
                Self::CURRENT
            }
        })
    }
    fn parse_str(value: &str) -> Option<Self> {
        // Ignore any suffixes such as "-dev" or "-nightly".
        let mut components = value.split('-').next().unwrap().splitn(3, '.');
        let major = components.next()?.parse().ok()?;
        let minor = components.next()?.parse().ok()?;
        let patch = components.next().unwrap_or("0").parse().ok()?;
        Some(RustcVersion { major, minor, patch })
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=8 | LINES=8 */

static CURRENT_OVERRIDABLE: OnceLock<RustcVersion> = OnceLock::new();

impl Display for RustcVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6 */

impl IntoDiagArg for RustcVersion {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(self.to_string()))
    }
}