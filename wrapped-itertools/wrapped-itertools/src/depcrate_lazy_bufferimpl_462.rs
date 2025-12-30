// Generated macro for impl_462 (impl)
macro_rules! Depcrate_lazy_bufferimpl_462 {
() => {
// Module: crate::lazy_buffer
// Provides: {"impl_462"}
// Dependencies: {}
impl < I > LazyBuffer < I > where I : Iterator , I :: Item : Clone , { pub fn get_at (& self , indices : & [usize]) -> Vec < I :: Item > { indices . iter () . map (| i | self . buffer [* i] . clone ()) . collect () } pub fn get_array < const K : usize > (& self , indices : [usize ; K]) -> [I :: Item ; K] { indices . map (| i | self . buffer [i] . clone ()) } }
};
}
