// Generated macro for emit_deletion (function)
macro_rules! Depcrate_index_functionemit_deletion {
() => {
// Module: crate::index::function
// Provides: {"emit_deletion"}
// Dependencies: {}
fn emit_deletion < 'rhs , 'lhs : 'rhs , E > ((idx , path , entry) : (usize , & 'lhs BStr , & 'lhs gix_index :: Entry) , mut cb : impl FnMut (ChangeRef < 'lhs , 'rhs >) -> Result < Action , E > , tracker : Option < & mut rewrites :: Tracker < ChangeRef < 'lhs , 'rhs > > > ,) -> Result < Action , Error > where E : Into < Box < dyn std :: error :: Error + Send + Sync > > , { let change = ChangeRef :: Deletion { location : Cow :: Borrowed (path) , index : idx , entry_mode : entry . mode , id : Cow :: Borrowed (entry . id . as_ref ()) , } ; let change = match tracker { None => change , Some (tracker) => match tracker . try_push_change (change , path) { Some (change) => change , None => return Ok (Action :: Continue) , } , } ; cb (change) . map_err (| err | Error :: Callback (err . into ())) }
};
}
