// Generated macro for impl_71 (impl)
macro_rules! Depcrate_inotifyimpl_71 {
() => {
// Module: crate::inotify
// Provides: {"impl_71"}
// Dependencies: {}
impl INotifyWatcher { fn from_event_handler (event_handler : Box < dyn EventHandler > , follow_links : bool ,) -> Result < Self > { let inotify = Inotify :: init () ? ; let event_loop = EventLoop :: new (inotify , event_handler , follow_links) ? ; let channel = event_loop . event_loop_tx . clone () ; let waker = event_loop . event_loop_waker . clone () ; event_loop . run () ; Ok (INotifyWatcher { channel , waker }) } fn watch_inner (& mut self , path : & Path , recursive_mode : RecursiveMode) -> Result < () > { let pb = if path . is_absolute () { path . to_owned () } else { let p = env :: current_dir () . map_err (Error :: io) ? ; p . join (path) } ; let (tx , rx) = unbounded () ; let msg = EventLoopMsg :: AddWatch (pb , recursive_mode , tx) ; self . channel . send (msg) . unwrap () ; self . waker . wake () . unwrap () ; rx . recv () . unwrap () } fn unwatch_inner (& mut self , path : & Path) -> Result < () > { let pb = if path . is_absolute () { path . to_owned () } else { let p = env :: current_dir () . map_err (Error :: io) ? ; p . join (path) } ; let (tx , rx) = unbounded () ; let msg = EventLoopMsg :: RemoveWatch (pb , tx) ; self . channel . send (msg) . unwrap () ; self . waker . wake () . unwrap () ; rx . recv () . unwrap () } }
};
}
