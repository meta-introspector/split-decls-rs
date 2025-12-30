// Generated macro for watchmap (function)
macro_rules! Depcrate_channelwatchmap {
() => {
// Module: crate::channel
// Provides: {"watchmap"}
// Dependencies: {}
# [test] fn watchmap () { let mut c = Channel :: get_private (BusType :: Session) . unwrap () ; c . set_watch_enabled (true) ; let w = c . watch () ; assert_eq ! (w . write , false) ; assert_eq ! (w . read , true) ; c . set_watch_enabled (false) ; println ! ("{:?}" , w) ; c . set_watch_enabled (true) ; }
};
}
