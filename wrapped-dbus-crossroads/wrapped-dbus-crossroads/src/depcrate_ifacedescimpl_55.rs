// Generated macro for impl_55 (impl)
macro_rules! Depcrate_ifacedescimpl_55 {
() => {
// Module: crate::ifacedesc
// Provides: {"impl_55"}
// Dependencies: {}
impl Annotations { pub fn insert < K : Into < String > , V : Into < String > > (& mut self , k : K , v : V) { let mut x = self . 0 . take () . unwrap_or_default () ; x . insert (k . into () , v . into ()) ; self . 0 = Some (x) ; } pub fn get (& self , key : & str) -> Option < & str > { self . 0 . as_ref () ? . get (key) . map (| x | & * * x) } fn is_empty (& self) -> bool { self . 0 . as_ref () . map (| s | s . len ()) . unwrap_or (0) == 0 } fn introspect (& self , prefix : & str) -> String { let mut r = String :: new () ; if let Some (anns) = & self . 0 { for (k , v) in anns . iter () { r += & format ! ("{}<annotation name=\"{}\" value=\"{}\"/>\n" , prefix , k , v) ; } } r } }
};
}
