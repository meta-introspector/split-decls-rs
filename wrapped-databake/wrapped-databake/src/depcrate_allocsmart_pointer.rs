// Generated macro for smart_pointer (macro)
macro_rules! Depcrate_allocsmart_pointer {
() => {
// Module: crate::alloc
// Provides: {"smart_pointer"}
// Dependencies: {}
macro_rules ! smart_pointer { ($ type : ty , $ constuctor : path) => { impl < T > Bake for $ type where T : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { ctx . insert ("alloc") ; let data = std :: ops :: Deref :: deref (self) . bake (ctx) ; quote ! { $ constuctor (# data) } } } } ; }
};
}
