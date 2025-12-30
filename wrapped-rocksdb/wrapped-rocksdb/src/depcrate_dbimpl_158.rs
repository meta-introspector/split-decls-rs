// Generated macro for impl_158 (impl)
macro_rules! Depcrate_dbimpl_158 {
() => {
// Module: crate::db
// Provides: {"impl_158"}
// Dependencies: {}
impl < I : DBInner > DBCommon < SingleThreaded , I > { # [doc = " Creates column family with given name and options"] pub fn create_cf < N : AsRef < str > > (& mut self , name : N , opts : & Options) -> Result < () , Error > { let inner = self . create_inner_cf_handle (name . as_ref () , opts) ? ; self . cfs . cfs . insert (name . as_ref () . to_string () , ColumnFamily { inner }) ; Ok (()) } # [doc = " Drops the column family with the given name"] pub fn drop_cf (& mut self , name : & str) -> Result < () , Error > { if let Some (cf) = self . cfs . cfs . remove (name) { self . drop_column_family (cf . inner , cf) } else { Err (Error :: new (format ! ("Invalid column family: {name}"))) } } # [doc = " Returns the underlying column family handle"] pub fn cf_handle (& self , name : & str) -> Option < & ColumnFamily > { self . cfs . cfs . get (name) } }
};
}
