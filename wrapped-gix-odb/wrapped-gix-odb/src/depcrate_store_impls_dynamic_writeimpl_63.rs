// Generated macro for impl_63 (impl)
macro_rules! Depcrate_store_impls_dynamic_writeimpl_63 {
() => {
// Module: crate::store_impls::dynamic::write
// Provides: {"impl_63"}
// Dependencies: {}
impl < S > gix_object :: Write for store :: Handle < S > where S : Deref < Target = dynamic :: Store > + Clone , { fn write_stream (& self , kind : Kind , size : u64 , from : & mut dyn Read) -> Result < ObjectId , gix_object :: write :: Error > { let mut snapshot = self . snapshot . borrow_mut () ; Ok (match snapshot . loose_dbs . first () { Some (ldb) => ldb . write_stream (kind , size , from) ? , None => { let new_snapshot = self . store . load_one_index (self . refresh , snapshot . marker) . map_err (Box :: new) ? . expect ("there is always at least one ODB, and this code runs only once for initialization") ; * snapshot = new_snapshot ; snapshot . loose_dbs [0] . write_stream (kind , size , from) ? } }) } }
};
}
