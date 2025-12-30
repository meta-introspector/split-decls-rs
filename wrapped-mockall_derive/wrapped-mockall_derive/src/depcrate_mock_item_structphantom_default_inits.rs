// Generated macro for phantom_default_inits (function)
macro_rules! Depcrate_mock_item_structphantom_default_inits {
() => {
// Module: crate::mock_item_struct
// Provides: {"phantom_default_inits"}
// Dependencies: {}
fn phantom_default_inits (generics : & Generics) -> Vec < TokenStream > { generics . params . iter () . enumerate () . map (| (count , _param) | { let phident = format_ident ! ("_t{count}") ; quote ! (# phident : :: std :: marker :: PhantomData) }) . collect () }
};
}
