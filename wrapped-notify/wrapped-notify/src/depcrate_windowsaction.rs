// Generated macro for Action (enum)
macro_rules! Depcrate_windowsAction {
() => {
// Module: crate::windows
// Provides: {"Action"}
// Dependencies: {}
enum Action { Watch (PathBuf , RecursiveMode) , Unwatch (PathBuf) , Stop , Configure (Config , BoundSender < Result < bool > >) , }
};
}
