// Generated macro for impl_944 (impl)
macro_rules! Depcrate_vtab_vtablogimpl_944 {
() => {
// Module: crate::vtab::vtablog
// Provides: {"impl_944"}
// Dependencies: {}
impl CreateVTab < '_ > for VTabLog { const KIND : VTabKind = VTabKind :: Default ; fn create (db : & mut VTabConnection , aux : Option < & Self :: Aux > , args : & [& [u8]] ,) -> Result < (String , Self) > { Self :: connect_create (db , aux , args , true) } fn destroy (& self) -> Result < () > { println ! ("VTabLog::destroy({})" , self . i_inst) ; Ok (()) } }
};
}
