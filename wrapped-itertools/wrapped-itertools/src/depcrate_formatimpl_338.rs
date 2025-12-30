// Generated macro for impl_338 (impl)
macro_rules! Depcrate_formatimpl_338 {
() => {
// Module: crate::format
// Provides: {"impl_338"}
// Dependencies: {}
impl < I , F > Clone for FormatWith < '_ , I , F > where (I , F) : Clone , { fn clone (& self) -> Self { struct PutBackOnDrop < 'r , 'a , I , F > { into : & 'r FormatWith < 'a , I , F > , inner : Option < (I , F) > , } impl < I , F > Drop for PutBackOnDrop < '_ , '_ , I , F > { fn drop (& mut self) { self . into . inner . set (self . inner . take ()) } } let pbod = PutBackOnDrop { inner : self . inner . take () , into : self , } ; Self { inner : Cell :: new (pbod . inner . clone ()) , sep : self . sep , } } }
};
}
