// Generated macro for impl_123 (impl)
macro_rules! Depcrate_helpersimpl_123 {
() => {
// Module: crate::helpers
// Provides: {"impl_123"}
// Dependencies: {}
# [doc = " implement `HelperDef` for bare function so we can use function as helper"] impl < F : for < 'reg , 'rc > Fn (& Helper < 'rc > , & 'reg Registry < 'reg > , & 'rc Context , & mut RenderContext < 'reg , 'rc > , & mut dyn Output ,) -> HelperResult , > HelperDef for F { fn call < 'reg : 'rc , 'rc > (& self , h : & Helper < 'rc > , r : & 'reg Registry < 'reg > , ctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > , out : & mut dyn Output ,) -> HelperResult { (* self) (h , r , ctx , rc , out) } }
};
}
