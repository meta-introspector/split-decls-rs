// Generated macro for impl_361 (impl)
macro_rules! Depcrate_object_tree_entryimpl_361 {
() => {
// Module: crate::object::tree::entry
// Provides: {"impl_361"}
// Dependencies: {}
impl std :: fmt :: Display for EntryRef < '_ , '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:>6o} {:>6} {}\t{}" , self . mode () , self . mode () . as_str () , self . id () . shorten_or_id () , self . filename ()) } }
};
}
