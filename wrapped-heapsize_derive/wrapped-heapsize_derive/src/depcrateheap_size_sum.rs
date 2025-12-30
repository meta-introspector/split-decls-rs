// Generated macro for heap_size_sum (function)
macro_rules! Depcrateheap_size_sum {
() => {
// Module: crate
// Provides: {"heap_size_sum"}
// Dependencies: {}
fn heap_size_sum (data : & Data) -> TokenStream { match * data { Data :: Struct (ref data) => { match data . fields { Fields :: Named (ref fields) => { let recurse = fields . named . iter () . map (| f | { let name = & f . ident ; quote_spanned ! { f . span () => heapsize :: HeapSize :: heap_size_of_children (& self .# name) } }) ; quote ! { 0 # (+ # recurse) * } } Fields :: Unnamed (ref fields) => { let recurse = fields . unnamed . iter () . enumerate () . map (| (i , f) | { let index = Index :: from (i) ; quote_spanned ! { f . span () => heapsize :: HeapSize :: heap_size_of_children (& self .# index) } }) ; quote ! { 0 # (+ # recurse) * } } Fields :: Unit => { quote ! (0) } } } Data :: Enum (_) | Data :: Union (_) => unimplemented ! () , } }
};
}
