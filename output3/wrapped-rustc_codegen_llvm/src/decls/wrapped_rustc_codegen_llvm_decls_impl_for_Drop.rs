use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Drop for ModuleLlvm {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.tm);
            llvm::LLVMContextDispose(&mut *(self.llcx as *mut _));
        }
    }
}
