// Generated macro for set (function)
macro_rules! Depcrate_stdimplset {
() => {
// Module: crate::stdimpl
// Provides: {"set"}
// Dependencies: {}
fn set (mut ctx : Context , cr : & mut Crossroads , (interface_name , property_name , _value) : (String , String , Variant < Box < dyn RefArg > >)) -> Option < Context > { let mut propctx = match ctx . check (| ctx | { PropContext :: new (cr , ctx . path () . clone () , interface_name , property_name) }) { Ok (p) => p , Err (_) => return Some (ctx) , } ; let ann = cr . registry () . find_annotation (propctx . iface_token , EMITS_CHANGED , Some (& propctx . name)) ; propctx . emits_changed = match ann { Some ("const") => Some ("const") , Some ("false") => Some ("false") , Some ("invalidates") => Some ("invalidates") , _ => Some ("true") , } ; propctx . context = Some (ctx) ; propctx . call_prop (cr , true) . map (| propctx | { propctx . context . unwrap () }) }
};
}
