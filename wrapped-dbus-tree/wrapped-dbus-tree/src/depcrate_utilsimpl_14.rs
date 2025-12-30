// Generated macro for impl_14 (impl)
macro_rules! Depcrate_utilsimpl_14 {
() => {
// Module: crate::utils
// Provides: {"impl_14"}
// Dependencies: {}
impl Annotations { pub fn new () -> Annotations { Annotations (None) } pub fn insert < N : Into < String > , V : Into < String > > (& mut self , n : N , v : V) { if self . 0 . is_none () { self . 0 = Some (BTreeMap :: new ()) } self . 0 . as_mut () . unwrap () . insert (n . into () , v . into ()) ; } pub fn introspect (& self , indent : & str) -> String { self . 0 . as_ref () . map (| s | s . iter () . fold ("" . into () , | aa , (ak , av) | { format ! ("{}{}<annotation name=\"{}\" value=\"{}\"/>\n" , aa , indent , ak , av) })) . unwrap_or_default () } }
};
}
