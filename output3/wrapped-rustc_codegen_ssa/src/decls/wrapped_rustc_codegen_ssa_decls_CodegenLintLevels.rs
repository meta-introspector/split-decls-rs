use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A list of lint levels used in codegen.
///
/// When using `-Z link-only`, we don't have access to the tcx and must work
/// solely from the `.rlink` file. `Lint`s are defined too early to be encodeable.
/// Instead, encode exactly the information we need.
#[derive(Copy, Clone, Debug, Encodable, Decodable)]
pub struct CodegenLintLevels {
    linker_messages: LevelAndSource,
}
