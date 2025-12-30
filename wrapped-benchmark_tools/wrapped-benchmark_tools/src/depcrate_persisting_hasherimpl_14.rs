// Generated macro for impl_14 (impl)
macro_rules! Depcrate_persisting_hasherimpl_14 {
() => {
// Module: crate::persisting_hasher
// Provides: {"impl_14"}
// Dependencies: {}
impl Default for PersistingHasherBuilder { fn default () -> Self { PersistingHasherBuilder { id : GLOBAL_COUNT . fetch_add (1 , Ordering :: SeqCst) , out : GLOBAL_OUT . get_or_init (| | Arc :: new (Mutex :: new (BufWriter :: new (File :: create (format ! ("hash_output-{}" , id ())) . unwrap ())))) . clone () , } } }
};
}
