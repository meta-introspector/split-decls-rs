// Generated macro for impl_104 (impl)
macro_rules! Depcrate_walkimpl_104 {
() => {
// Module: crate::walk
// Provides: {"impl_104"}
// Dependencies: {}
impl Iterator for WalkEventIter { type Item = walkdir :: Result < WalkEvent > ; # [inline (always)] fn next (& mut self) -> Option < walkdir :: Result < WalkEvent > > { let dent = self . next . take () . or_else (| | self . it . next ()) ; let depth = match dent { None => 0 , Some (Ok (ref dent)) => dent . depth () , Some (Err (ref err)) => err . depth () , } ; if depth < self . depth { self . depth -= 1 ; self . next = dent ; return Some (Ok (WalkEvent :: Exit)) ; } self . depth = depth ; match dent { None => None , Some (Err (err)) => Some (Err (err)) , Some (Ok (dent)) => { if walkdir_is_dir (& dent) { self . depth += 1 ; Some (Ok (WalkEvent :: Dir (dent))) } else { Some (Ok (WalkEvent :: File (dent))) } } } } }
};
}
