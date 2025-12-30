// Generated macro for impl_380 (impl)
macro_rules! Depcrate_groupbylazyimpl_380 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_380"}
// Dependencies: {}
impl < 'a , I > Iterator for Chunks < 'a , I > where I : Iterator , I :: Item : 'a , { type Item = Chunk < 'a , I > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let index = self . parent . index . get () ; self . parent . index . set (index + 1) ; let inner = & mut * self . parent . inner . borrow_mut () ; inner . step (index) . map (| elt | Chunk { parent : self . parent , index , first : Some (elt) , }) } }
};
}
