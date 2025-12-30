// Generated macro for impl_95 (impl)
macro_rules! Depcrate_kqueueimpl_95 {
() => {
// Module: crate::kqueue
// Provides: {"impl_95"}
// Dependencies: {}
impl KqueueWatcher { fn from_event_handler (event_handler : Box < dyn EventHandler > , follow_symlinks : bool ,) -> Result < Self > { let kqueue = kqueue :: Watcher :: new () ? ; let event_loop = EventLoop :: new (kqueue , event_handler , follow_symlinks) ? ; let channel = event_loop . event_loop_tx . clone () ; let waker = event_loop . event_loop_waker . clone () ; event_loop . run () ; Ok (KqueueWatcher { channel , waker }) } fn watch_inner (& mut self , path : & Path , recursive_mode : RecursiveMode) -> Result < () > { let pb = if path . is_absolute () { path . to_owned () } else { let p = env :: current_dir () . map_err (Error :: io) ? ; p . join (path) } ; let (tx , rx) = unbounded () ; let msg = EventLoopMsg :: AddWatch (pb , recursive_mode , tx) ; self . channel . send (msg) . map_err (| e | Error :: generic (& e . to_string ())) ? ; self . waker . wake () . map_err (| e | Error :: generic (& e . to_string ())) ? ; rx . recv () . unwrap () . map_err (| e | Error :: generic (& e . to_string ())) } fn unwatch_inner (& mut self , path : & Path) -> Result < () > { let pb = if path . is_absolute () { path . to_owned () } else { let p = env :: current_dir () . map_err (Error :: io) ? ; p . join (path) } ; let (tx , rx) = unbounded () ; let msg = EventLoopMsg :: RemoveWatch (pb , tx) ; self . channel . send (msg) . map_err (| e | Error :: generic (& e . to_string ())) ? ; self . waker . wake () . map_err (| e | Error :: generic (& e . to_string ())) ? ; rx . recv () . unwrap () . map_err (| e | Error :: generic (& e . to_string ())) } }
};
}
