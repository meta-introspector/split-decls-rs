// Generated macro for impl_159 (impl)
macro_rules! Depcrateimpl_159 {
() => {
// Module: crate
// Provides: {"impl_159"}
// Dependencies: {}
impl Iterator for HunkIter < '_ > { type Item = Hunk ; fn next (& mut self) -> Option < Self :: Item > { loop { let removed = (& mut self . removed) . take_while (| & & removed | removed) . count () as u32 ; let added = (& mut self . added) . take_while (| & & added | added) . count () as u32 ; if removed != 0 || added != 0 { let start_before = self . pos_before ; let start_after = self . pos_after ; self . pos_before += removed ; self . pos_after += added ; let hunk = Hunk { before : start_before .. self . pos_before , after : start_after .. self . pos_after , } ; self . pos_before += 1 ; self . pos_after += 1 ; return Some (hunk) ; } else if self . removed . len () == 0 && self . added . len () == 0 { return None ; } else { self . pos_before += 1 ; self . pos_after += 1 ; } } } }
};
}
