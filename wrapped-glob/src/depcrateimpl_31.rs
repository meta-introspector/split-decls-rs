// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl Iterator for Paths { type Item = GlobResult ; fn next (& mut self) -> Option < GlobResult > { if let Some (scope) = self . scope . take () { if ! self . dir_patterns . is_empty () { assert ! (self . dir_patterns . len () < usize :: MAX) ; fill_todo (& mut self . todo , & self . dir_patterns , 0 , & scope , self . options) ; } } loop { if self . dir_patterns . is_empty () || self . todo . is_empty () { return None ; } let (path , mut idx) = match self . todo . pop () . unwrap () { Ok (pair) => pair , Err (e) => return Some (Err (e)) , } ; if idx == usize :: MAX { if self . require_dir && ! path . is_directory { continue ; } return Some (Ok (path . into_path ())) ; } if self . dir_patterns [idx] . is_recursive { let mut next = idx ; while (next + 1) < self . dir_patterns . len () && self . dir_patterns [next + 1] . is_recursive { next += 1 ; } if path . is_directory { fill_todo (& mut self . todo , & self . dir_patterns , next , & path , self . options ,) ; if next == self . dir_patterns . len () - 1 { return Some (Ok (path . into_path ())) ; } else { idx = next + 1 ; } } else if next == self . dir_patterns . len () - 1 { continue ; } else { idx = next + 1 ; } } if self . dir_patterns [idx] . matches_with ({ match path . file_name () . and_then (| s | s . to_str ()) { None => continue , Some (x) => x , } } , self . options ,) { if idx == self . dir_patterns . len () - 1 { if ! self . require_dir || path . is_directory { return Some (Ok (path . into_path ())) ; } } else { fill_todo (& mut self . todo , & self . dir_patterns , idx + 1 , & path , self . options ,) ; } } } } }
};
}
