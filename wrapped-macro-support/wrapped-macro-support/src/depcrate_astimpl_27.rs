// Generated macro for impl_27 (impl)
macro_rules! Depcrate_astimpl_27 {
() => {
// Module: crate::ast
// Provides: {"impl_27"}
// Dependencies: {}
impl Hash for ImportModule { fn hash < H : Hasher > (& self , h : & mut H) { match self { ImportModule :: Named (name , _) => (1u8 , name) . hash (h) , ImportModule :: Inline (idx) => (2u8 , idx) . hash (h) , ImportModule :: RawNamed (name , _) => (3u8 , name) . hash (h) , } } }
};
}
