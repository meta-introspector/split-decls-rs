// Generated macro for Cache (struct)
macro_rules! Depcrate_utils_cacheCache {
() => {
// Module: crate::utils::cache
// Provides: {"Cache"}
// Dependencies: {}
# [doc = " This is essentially a `HashMap` which allows storing any type in its input and"] # [doc = " any type in its output. It is a write-once cache; values are never evicted,"] # [doc = " which means that references to the value can safely be returned from the"] # [doc = " `get()` method."] # [derive (Debug , Default)] pub struct Cache { cache : RefCell < HashMap < TypeId , Box < dyn Any > , > , > , # [cfg (test)] # [doc = " Contains step metadata of executed steps (in the same order in which they were executed)."] # [doc = " Useful for tests."] executed_steps : RefCell < Vec < ExecutedStep > > , }
};
}
