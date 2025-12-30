// Generated macro for impl_1136 (impl)
macro_rules! Depcrate_fs_memimpl_1136 {
() => {
// Module: crate::fs::mem
// Provides: {"impl_1136"}
// Dependencies: {}
impl RomFileInterface { pub fn new (inner : Arc < RwLock < RomFileInner > >) -> Self { Self { pos : Arc :: new (Mutex :: new (0)) , inner , } } pub fn len (& self) -> usize { block_on (async { Ok (self . inner . read () . await . data . len ()) } , None) . unwrap () } }
};
}
