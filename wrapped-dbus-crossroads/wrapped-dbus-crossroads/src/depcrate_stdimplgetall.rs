// Generated macro for getall (function)
macro_rules! Depcrate_stdimplgetall {
() => {
// Module: crate::stdimpl
// Provides: {"getall"}
// Dependencies: {}
fn getall (mut ctx : Context , cr : & mut Crossroads , (interface_name ,) : (String ,)) -> Option < Context > { if interface_name == "" { return getall_all (ctx , cr) ; } let mut propctx = match ctx . check (| ctx | { PropContext :: new (cr , ctx . path () . clone () , interface_name , "" . into ()) }) { Ok (p) => p , Err (_) => return Some (ctx) , } ; propctx . context = Some (ctx) ; propctx . call_all_props (cr , move | pactx | { let pctx = pactx . propctx . as_mut () . unwrap () ; let answers = & pactx . answers ; pctx . context . as_mut () . unwrap () . do_reply (| msg | { msg . append_all ((answers ,)) ; }) ; }) . map (| propctx | { propctx . context . unwrap () }) }
};
}
