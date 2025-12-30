// Generated macro for impls (module)
macro_rules! Depcrate_fileimpls {
() => {
// Module: crate::file
// Provides: {"impls"}
// Dependencies: {}
mod impls { use std :: ops :: { Deref , DerefMut } ; use crate :: { File , State } ; impl Deref for File { type Target = State ; fn deref (& self) -> & Self :: Target { & self . state } } impl DerefMut for File { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . state } } }
};
}
