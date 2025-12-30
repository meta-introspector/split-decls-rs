// Generated macro for ReadDirCallback (type)
macro_rules! Depcrate_core_read_dir_iterReadDirCallback {
() => {
// Module: crate::core::read_dir_iter
// Provides: {"ReadDirCallback"}
// Dependencies: {}
# [doc = " Client's read dir function."] pub (crate) type ReadDirCallback < C > = dyn Fn (ReadDirSpec < C >) -> Result < ReadDir < C > > + Send + Sync + 'static ;
};
}
