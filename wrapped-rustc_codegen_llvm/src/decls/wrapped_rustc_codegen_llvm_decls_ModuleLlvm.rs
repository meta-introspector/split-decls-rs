use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct ModuleLlvm {
    llcx: &'static mut llvm::Context,
    llmod_raw: *const llvm::Module,
    tm: ManuallyDrop<OwnedTargetMachine>,
}
