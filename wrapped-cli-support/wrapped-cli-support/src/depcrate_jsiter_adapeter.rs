// Generated macro for iter_adapeter (function)
macro_rules! Depcrate_jsiter_adapeter {
() => {
// Module: crate::js
// Provides: {"iter_adapeter"}
// Dependencies: {}
# [doc = " Iterate over the adapters in a deterministic order."] fn iter_adapeter < 'a > (aux : & 'a WasmBindgenAux , wit : & 'a NonstandardWitSection , module : & Module ,) -> Vec < (AdapterId , & 'a Adapter , ContextAdapterKind < 'a >) > { let mut adapters : Vec < _ > = wit . adapters . iter () . map (| (id , adapter) | { let kind = ContextAdapterKind :: get (* id , aux , wit) ; (* id , adapter , kind) }) . collect () ; adapters . sort_by (| (_ , _ , a) , (_ , _ , b) | { fn get_kind_order (kind : & ContextAdapterKind) -> u8 { match kind { ContextAdapterKind :: Import (_) => 0 , ContextAdapterKind :: Export (_) => 1 , ContextAdapterKind :: Adapter => 2 , } } match (a , b) { (ContextAdapterKind :: Import (a) , ContextAdapterKind :: Import (b)) => { let a = module . imports . get (* a) ; let b = module . imports . get (* b) ; a . name . cmp (& b . name) } _ => get_kind_order (a) . cmp (& get_kind_order (b)) , } }) ; adapters }
};
}
