// Generated macro for ProcessReadDirFunction (type)
macro_rules! DepcrateProcessReadDirFunction {
() => {
// Module: crate
// Provides: {"ProcessReadDirFunction"}
// Dependencies: {}
type ProcessReadDirFunction < C > = dyn Fn (Option < usize > , & Path , & mut < C as ClientState > :: ReadDirState , & mut Vec < Result < DirEntry < C > > >) + Send + Sync + 'static ;
};
}
