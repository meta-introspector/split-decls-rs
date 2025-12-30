// Generated macro for impl_24 (impl)
macro_rules! Depcrate_uniximpl_24 {
() => {
// Module: crate::unix
// Provides: {"impl_24"}
// Dependencies: {}
impl CommandExt for Command { fn uid (& mut self , id : UserId) -> & mut Command { self . inner . uid (id) ; self } fn gid (& mut self , id : GroupId) -> & mut Command { self . inner . gid (id) ; self } fn exec (& mut self) -> io :: Error { self . inner . exec () } fn arg0 < S > (& mut self , arg : S) -> & mut Command where S : AsRef < OsStr > , { self . inner . arg0 (arg) ; self } }
};
}
