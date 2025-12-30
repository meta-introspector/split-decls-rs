// Generated macro for WinconStream (struct)
macro_rules! Depcrate_winconWinconStream {
() => {
// Module: crate::wincon
// Provides: {"WinconStream"}
// Dependencies: {}
# [doc = " Only pass printable data to the inner `Write`"] # [cfg (feature = "wincon")] # [derive (Debug)] pub struct WinconStream < S > where S : anstyle_wincon :: WinconStream , { raw : S , state : Box < WinconBytes > , }
};
}
