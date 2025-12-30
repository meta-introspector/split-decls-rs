// Generated macro for Project (struct)
macro_rules! Depcrate_prjProject {
() => {
// Module: crate::prj
// Provides: {"Project"}
// Dependencies: {}
pub struct Project { pub name : OsString , root : PathBuf , channel : Option < Channel > , nocapture : bool , ws : Arc < std :: sync :: RwLock < () > > , default_timeout : Option < u64 > , }
};
}
