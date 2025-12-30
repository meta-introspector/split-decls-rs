// Generated macro for macro_path (function)
macro_rules! Depcratemacro_path {
() => {
// Module: crate
// Provides: {"macro_path"}
// Dependencies: {}
fn macro_path (module : & str , name : & str) -> impl Iterator < Item = TokenTree > { [Punct :: new (':' , Spacing :: Joint) . into () , Punct :: new (':' , Spacing :: Alone) . into () , Ident :: new (module , Span :: call_site ()) . into () , Punct :: new (':' , Spacing :: Joint) . into () , Punct :: new (':' , Spacing :: Alone) . into () , Ident :: new (name , Span :: call_site ()) . into () , Punct :: new ('!' , Spacing :: Alone) . into () ,] . into_iter () }
};
}
