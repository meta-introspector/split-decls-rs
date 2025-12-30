// Generated macro for check (function)
macro_rules! Depcrate_repository_mailmapcheck {
() => {
// Module: crate::repository::mailmap
// Provides: {"check"}
// Dependencies: {}
pub fn check (repo : gix :: Repository , format : OutputFormat , contacts : Vec < BString > , mut out : impl io :: Write , mut err : impl io :: Write ,) -> anyhow :: Result < () > { if format != OutputFormat :: Human { bail ! ("Only human output is supported right now") ; } if contacts . is_empty () { bail ! ("specify at least one contact to run through the mailmap") } let mut mailmap = gix :: mailmap :: Snapshot :: default () ; if let Err (err) = repo . open_mailmap_into (& mut mailmap) { bail ! (err) ; } let mut buf = Vec :: new () ; for contact in contacts { let actor = match gix :: actor :: IdentityRef :: from_bytes :: < () > (& contact) { Ok (a) => a , Err (_) => { let Some (email) = contact . trim_start () . strip_prefix (b"<") . and_then (| rest | rest . trim_end () . strip_suffix (b">")) else { writeln ! (err , "Failed to parse contact '{contact}' - skipping") ? ; continue ; } ; gix :: actor :: IdentityRef { name : "" . into () , email : email . into () , } } } ; let resolved = mailmap . resolve_cow (gix :: actor :: SignatureRef { name : actor . name , email : actor . email , time : Default :: default () , }) ; let resolved = gix :: actor :: IdentityRef { name : resolved . name . as_ref () , email : resolved . email . as_ref () , } ; buf . clear () ; resolved . write_to (& mut buf) ? ; out . write_all (& buf) ? ; out . write_all (b"\n") ? ; } Ok (()) }
};
}
