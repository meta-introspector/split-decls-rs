use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct ModuleCodegen<M> {
    /// The name of the module. When the crate may be saved between
    /// compilations, incremental compilation requires that name be
    /// unique amongst **all** crates. Therefore, it should contain
    /// something unique to this crate (e.g., a module path) as well
    /// as the crate name and disambiguator.
    /// We currently generate these names via CodegenUnit::build_cgu_name().
    pub name: String,
    pub module_llvm: M,
    pub kind: ModuleKind,
    /// Saving the ThinLTO buffer for embedding in the object file.
    pub thin_lto_buffer: Option<Vec<u8>>,
}
