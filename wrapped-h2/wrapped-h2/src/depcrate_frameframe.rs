// Generated macro for Frame (enum)
macro_rules! Depcrate_frameFrame {
() => {
// Module: crate::frame
// Provides: {"Frame"}
// Dependencies: {}
# [derive (Eq , PartialEq)] pub enum Frame < T = Bytes > { Data (Data < T >) , Headers (Headers) , Priority (Priority) , PushPromise (PushPromise) , Settings (Settings) , Ping (Ping) , GoAway (GoAway) , WindowUpdate (WindowUpdate) , Reset (Reset) , }
};
}
