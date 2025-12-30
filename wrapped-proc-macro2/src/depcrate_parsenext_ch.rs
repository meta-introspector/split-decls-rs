// Generated macro for next_ch (macro)
macro_rules! Depcrate_parsenext_ch {
() => {
// Module: crate::parse
// Provides: {"next_ch"}
// Dependencies: {}
macro_rules ! next_ch { ($ chars : ident @ $ pat : pat) => { match $ chars . next () { Some ((_ , ch)) => match ch { $ pat => ch , _ => return Err (Reject) , } , None => return Err (Reject) , } } ; }
};
}
