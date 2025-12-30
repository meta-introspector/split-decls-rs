// Generated macro for MediaKeyCode (enum)
macro_rules! Depcrate_eventMediaKeyCode {
() => {
// Module: crate::event
// Provides: {"MediaKeyCode"}
// Dependencies: {}
# [doc = " Represents a media key (as part of [`KeyCode::Media`])."] # [derive (Debug , PartialOrd , Ord , PartialEq , Eq , Clone , Copy , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum MediaKeyCode { # [doc = " Play media key."] Play , # [doc = " Pause media key."] Pause , # [doc = " Play/Pause media key."] PlayPause , # [doc = " Reverse media key."] Reverse , # [doc = " Stop media key."] Stop , # [doc = " Fast-forward media key."] FastForward , # [doc = " Rewind media key."] Rewind , # [doc = " Next-track media key."] TrackNext , # [doc = " Previous-track media key."] TrackPrevious , # [doc = " Record media key."] Record , # [doc = " Lower-volume media key."] LowerVolume , # [doc = " Raise-volume media key."] RaiseVolume , # [doc = " Mute media key."] MuteVolume , }
};
}
