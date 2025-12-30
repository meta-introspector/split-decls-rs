// Generated macro for skip_filesize (function)
macro_rules! Depcrate_walkskip_filesize {
() => {
// Module: crate::walk
// Provides: {"skip_filesize"}
// Dependencies: {}
fn skip_filesize (max_filesize : u64 , path : & Path , ent : & Option < Metadata > ,) -> bool { let filesize = match * ent { Some (ref md) => Some (md . len ()) , None => None , } ; if let Some (fs) = filesize { if fs > max_filesize { log :: debug ! ("ignoring {}: {} bytes" , path . display () , fs) ; true } else { false } } else { false } }
};
}
