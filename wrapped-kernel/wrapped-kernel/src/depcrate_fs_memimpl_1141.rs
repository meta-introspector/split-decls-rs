// Generated macro for impl_1141 (impl)
macro_rules! Depcrate_fs_memimpl_1141 {
() => {
// Module: crate::fs::mem
// Provides: {"impl_1141"}
// Dependencies: {}
impl RamFileInterface { pub fn new (inner : Arc < RwLock < RamFileInner > >) -> Self { Self { pos : Arc :: new (Mutex :: new (0)) , inner , } } pub fn len (& self) -> usize { block_on (async { Ok (self . inner . read () . await . data . len ()) } , None) . unwrap () } }
};
}
