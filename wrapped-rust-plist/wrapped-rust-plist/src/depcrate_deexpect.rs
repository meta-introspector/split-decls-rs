// Generated macro for expect (macro)
macro_rules! Depcrate_deexpect {
() => {
// Module: crate::de
// Provides: {"expect"}
// Dependencies: {}
macro_rules ! expect { ($ next : expr , $ kind : expr) => { match $ next { Some (Ok (ref event)) if EventKind :: of_event (event) != $ kind => { return Err (error :: unexpected_event_type ($ kind , event)) ?; } Some (Ok (event)) => event , Some (Err (err)) => return Err (err) , None => return Err (ErrorKind :: UnexpectedEndOfEventStream . without_position ()) , } } ; }
};
}
