// Generated macro for impl_339 (impl)
macro_rules! Depcrate_formatimpl_339 {
() => {
// Module: crate::format
// Provides: {"impl_339"}
// Dependencies: {}
impl < I > Clone for Format < '_ , I > where I : Clone , { fn clone (& self) -> Self { struct PutBackOnDrop < 'r , 'a , I > { into : & 'r Format < 'a , I > , inner : Option < I > , } impl < I > Drop for PutBackOnDrop < '_ , '_ , I > { fn drop (& mut self) { self . into . inner . set (self . inner . take ()) } } let pbod = PutBackOnDrop { inner : self . inner . take () , into : self , } ; Self { inner : Cell :: new (pbod . inner . clone ()) , sep : self . sep , } } }
};
}
