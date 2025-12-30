// Generated macro for MemoryHistory (struct)
macro_rules! Depcrate_memoryMemoryHistory {
() => {
// Module: crate::memory
// Provides: {"MemoryHistory"}
// Dependencies: {}
# [doc = " A [`History`] that is implemented with in memory history stack and is usable in most targets."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " MemoryHistory does not support relative paths and will panic if routes are not starting with `/`."] # [derive (Clone , Default)] pub struct MemoryHistory { inner : Rc < RefCell < LocationStack > > , callbacks : Rc < RefCell < Vec < WeakCallback > > > , }
};
}
