// Generated macro for impl_143 (impl)
macro_rules! Depcrate_jsimpl_143 {
() => {
// Module: crate::js
// Provides: {"impl_143"}
// Dependencies: {}
impl < 'a > ContextAdapterKind < 'a > { fn get (id : AdapterId , aux : & 'a WasmBindgenAux , wit : & 'a NonstandardWitSection) -> Self { match aux . export_map . get (& id) { Some (export) => ContextAdapterKind :: Export (export) , None => { let core = wit . implements . iter () . find (| pair | pair . 2 == id) ; match core { Some ((core , _ , _)) => ContextAdapterKind :: Import (* core) , None => ContextAdapterKind :: Adapter , } } } } }
};
}
