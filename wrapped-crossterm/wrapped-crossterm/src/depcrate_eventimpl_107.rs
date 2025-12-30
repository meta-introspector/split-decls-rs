// Generated macro for impl_107 (impl)
macro_rules! Depcrate_eventimpl_107 {
() => {
// Module: crate::event
// Provides: {"impl_107"}
// Dependencies: {}
impl Display for MediaKeyCode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MediaKeyCode :: Play => write ! (f , "Play") , MediaKeyCode :: Pause => write ! (f , "Pause") , MediaKeyCode :: PlayPause => write ! (f , "Play/Pause") , MediaKeyCode :: Reverse => write ! (f , "Reverse") , MediaKeyCode :: Stop => write ! (f , "Stop") , MediaKeyCode :: FastForward => write ! (f , "Fast Forward") , MediaKeyCode :: Rewind => write ! (f , "Rewind") , MediaKeyCode :: TrackNext => write ! (f , "Next Track") , MediaKeyCode :: TrackPrevious => write ! (f , "Previous Track") , MediaKeyCode :: Record => write ! (f , "Record") , MediaKeyCode :: LowerVolume => write ! (f , "Lower Volume") , MediaKeyCode :: RaiseVolume => write ! (f , "Raise Volume") , MediaKeyCode :: MuteVolume => write ! (f , "Mute Volume") , } } }
};
}
