// Generated macro for toolchain_channel (function)
macro_rules! Depcratetoolchain_channel {
() => {
// Module: crate
// Provides: {"toolchain_channel"}
// Dependencies: {}
fn toolchain_channel (db : & dyn RootQueryDb , krate : Crate) -> Option < ReleaseChannel > { krate . workspace_data (db) . toolchain . as_ref () . and_then (| v | ReleaseChannel :: from_str (& v . pre)) }
};
}
