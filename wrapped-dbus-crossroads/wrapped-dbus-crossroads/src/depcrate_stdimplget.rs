// Generated macro for get (function)
macro_rules! Depcrate_stdimplget {
() => {
// Module: crate::stdimpl
// Provides: {"get"}
// Dependencies: {}
fn get (mut ctx : Context , cr : & mut Crossroads , (interface_name , property_name) : (String , String)) -> Option < Context > { let mut propctx = match ctx . check (| ctx | { PropContext :: new (cr , ctx . path () . clone () , interface_name , property_name) }) { Ok (p) => p , Err (_) => return Some (ctx) , } ; propctx . context = Some (ctx) ; propctx . call_prop (cr , false) . map (| propctx | { propctx . context . unwrap () }) }
};
}
