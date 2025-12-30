// Generated macro for handle_error (function)
macro_rules! Depcrate_checkout_chunkhandle_error {
() => {
// Module: crate::checkout::chunk
// Provides: {"handle_error"}
// Dependencies: {}
fn handle_error < E > (err : E , entry_path : & BStr , files : & AtomicUsize , errors : & mut Vec < checkout :: ErrorRecord > , keep_going : bool ,) -> Result < () , E > where E : std :: error :: Error + Send + Sync + 'static , { if keep_going { errors . push (checkout :: ErrorRecord { path : entry_path . into () , error : Box :: new (err) , }) ; files . fetch_add (1 , Ordering :: Relaxed) ; Ok (()) } else { Err (err) } }
};
}
