// Generated macro for impl_31 (impl)
macro_rules! Depcrate_fs_dirimpl_31 {
() => {
// Module: crate::fs::dir
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'dir , 'ig > Iterator for Files < 'dir , 'ig > { type Item = File < 'dir > ; fn next (& mut self) -> Option < Self :: Item > { match self . dots { DotsNext :: Dot => { self . dots = DotsNext :: DotDot ; Some (File :: new_aa_current (self . dir , self . total_size)) } DotsNext :: DotDot => { self . dots = DotsNext :: Files ; Some (File :: new_aa_parent (self . parent () , self . dir , self . total_size ,)) } DotsNext :: Files => self . next_visible_file () , } } }
};
}
