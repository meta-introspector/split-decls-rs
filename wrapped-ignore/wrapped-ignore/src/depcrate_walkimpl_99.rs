// Generated macro for impl_99 (impl)
macro_rules! Depcrate_walkimpl_99 {
() => {
// Module: crate::walk
// Provides: {"impl_99"}
// Dependencies: {}
impl Iterator for Walk { type Item = Result < DirEntry , Error > ; # [inline (always)] fn next (& mut self) -> Option < Result < DirEntry , Error > > { loop { let ev = match self . it . as_mut () . and_then (| it | it . next ()) { Some (ev) => ev , None => { match self . its . next () { None => return None , Some ((_ , None)) => { return Some (Ok (DirEntry :: new_stdin ())) ; } Some ((path , Some (it))) => { self . it = Some (it) ; if path . is_dir () { let (ig , err) = self . ig_root . add_parents (path) ; self . ig = ig ; if let Some (err) = err { return Some (Err (err)) ; } } else { self . ig = self . ig_root . clone () ; } } } continue ; } } ; match ev { Err (err) => { return Some (Err (Error :: from_walkdir (err))) ; } Ok (WalkEvent :: Exit) => { self . ig = self . ig . parent () . unwrap () ; } Ok (WalkEvent :: Dir (ent)) => { let mut ent = DirEntry :: new_walkdir (ent , None) ; let should_skip = match self . skip_entry (& ent) { Err (err) => return Some (Err (err)) , Ok (should_skip) => should_skip , } ; if should_skip { self . it . as_mut () . unwrap () . it . skip_current_dir () ; let (igtmp , _) = self . ig . add_child (ent . path ()) ; self . ig = igtmp ; continue ; } let (igtmp , err) = self . ig . add_child (ent . path ()) ; self . ig = igtmp ; ent . err = err ; return Some (Ok (ent)) ; } Ok (WalkEvent :: File (ent)) => { let ent = DirEntry :: new_walkdir (ent , None) ; let should_skip = match self . skip_entry (& ent) { Err (err) => return Some (Err (err)) , Ok (should_skip) => should_skip , } ; if should_skip { continue ; } return Some (Ok (ent)) ; } } } } }
};
}
