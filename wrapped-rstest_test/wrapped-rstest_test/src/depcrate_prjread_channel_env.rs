// Generated macro for read_channel_env (function)
macro_rules! Depcrate_prjread_channel_env {
() => {
// Module: crate::prj
// Provides: {"read_channel_env"}
// Dependencies: {}
fn read_channel_env () -> Option < Channel > { std :: env :: var (ENV_CHANNEL) . ok () . map (Channel :: from) }
};
}
