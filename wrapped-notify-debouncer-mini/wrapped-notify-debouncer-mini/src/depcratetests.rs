// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use notify :: RecursiveMode ; use std :: fs ; use tempfile :: tempdir ; # [test] fn integration () -> Result < () , Box < dyn std :: error :: Error > > { let dir = tempdir () ? ; let (tx , rx) = std :: sync :: mpsc :: channel () ; let mut debouncer = new_debouncer (Duration :: from_secs (1) , tx) ? ; debouncer . watcher () . watch (dir . path () , RecursiveMode :: Recursive) ? ; let file_path = dir . path () . join ("file.txt") ; fs :: write (& file_path , b"Lorem ipsum") ? ; println ! ("waiting for event at {}" , file_path . display ()) ; let deadline = Instant :: now () + Duration :: from_secs (10) ; while deadline > Instant :: now () { let events = rx . recv_timeout (deadline - Instant :: now ()) . expect ("did not receive expected event") . expect ("received an error") ; for event in events { if event == DebouncedEvent :: new (file_path . clone () , DebouncedEventKind :: Any) || event == DebouncedEvent :: new (file_path . canonicalize () ? , DebouncedEventKind :: Any) { return Ok (()) ; } println ! ("unexpected event: {event:?}") ; } } panic ! ("did not receive expected event") ; } }
};
}
