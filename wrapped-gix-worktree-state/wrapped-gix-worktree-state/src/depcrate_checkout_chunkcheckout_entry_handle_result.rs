// Generated macro for checkout_entry_handle_result (function)
macro_rules! Depcrate_checkout_chunkcheckout_entry_handle_result {
() => {
// Module: crate::checkout::chunk
// Provides: {"checkout_entry_handle_result"}
// Dependencies: {}
pub fn checkout_entry_handle_result < 'entry , Find > (entry : & 'entry mut gix_index :: Entry , entry_path : & 'entry BStr , errors : & mut Vec < checkout :: ErrorRecord > , collisions : & mut Vec < checkout :: Collision > , files : & AtomicUsize , bytes : & AtomicUsize , Context { objects , path_cache , filters , buf , options , } : & mut Context < Find > ,) -> Result < entry :: Outcome < 'entry > , checkout :: Error > where Find : gix_object :: Find + Clone , { let res = entry :: checkout (entry , entry_path , entry :: Context { objects , path_cache , filters , buf , } , * options ,) ; match res { Ok (out) => { if let Some (num) = out . as_bytes () { bytes . fetch_add (num , Ordering :: Relaxed) ; files . fetch_add (1 , Ordering :: Relaxed) ; } Ok (out) } Err (checkout :: Error :: Io (err)) if is_collision (& err , entry_path , collisions , files) => { Ok (entry :: Outcome :: Written { bytes : 0 }) } Err (err) => handle_error (err , entry_path , files , errors , options . keep_going) . map (| () | entry :: Outcome :: Written { bytes : 0 }) , } }
};
}
