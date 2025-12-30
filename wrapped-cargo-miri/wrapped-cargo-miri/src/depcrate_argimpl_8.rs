// Generated macro for impl_8 (impl)
macro_rules! Depcrate_argimpl_8 {
() => {
// Module: crate::arg
// Provides: {"impl_8"}
// Dependencies: {}
impl < 's , I : Iterator < Item = Cow < 's , str > > > Iterator for ArgSplitFlagValue < '_ , I > { type Item = Result < Cow < 's , str > , Cow < 's , str > > ; fn next (& mut self) -> Option < Self :: Item > { let Some (args) = self . args . as_mut () else { return None ; } ; let arg = args . next () ? ; if arg == "--" { self . args = None ; return Some (Err (Cow :: Borrowed ("--"))) ; } match & arg { Cow :: Borrowed (arg) => if let Some (suffix) = arg . strip_prefix (self . name) { if suffix . is_empty () { return args . next () . map (Ok) ; } else if let Some (suffix) = suffix . strip_prefix ('=') { return Some (Ok (Cow :: Borrowed (suffix))) ; } } , Cow :: Owned (arg) => if let Some (suffix) = arg . strip_prefix (self . name) { if suffix . is_empty () { return args . next () . map (Ok) ; } else if let Some (suffix) = suffix . strip_prefix ('=') { return Some (Ok (Cow :: Owned (suffix . to_owned ()))) ; } } , } Some (Err (arg)) } }
};
}
