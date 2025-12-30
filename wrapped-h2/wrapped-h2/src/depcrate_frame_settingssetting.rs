// Generated macro for Setting (enum)
macro_rules! Depcrate_frame_settingsSetting {
() => {
// Module: crate::frame::settings
// Provides: {"Setting"}
// Dependencies: {}
# [doc = " An enum that lists all valid settings that can be sent in a SETTINGS"] # [doc = " frame."] # [doc = ""] # [doc = " Each setting has a value that is a 32 bit unsigned integer (6.5.1.)."] # [derive (Debug)] pub enum Setting { HeaderTableSize (u32) , EnablePush (u32) , MaxConcurrentStreams (u32) , InitialWindowSize (u32) , MaxFrameSize (u32) , MaxHeaderListSize (u32) , EnableConnectProtocol (u32) , }
};
}
