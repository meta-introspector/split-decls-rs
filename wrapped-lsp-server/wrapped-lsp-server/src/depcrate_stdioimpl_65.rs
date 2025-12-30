// Generated macro for impl_65 (impl)
macro_rules! Depcrate_stdioimpl_65 {
() => {
// Module: crate::stdio
// Provides: {"impl_65"}
// Dependencies: {}
impl IoThreads { pub fn join (self) -> io :: Result < () > { match self . reader . join () { Ok (r) => r ? , Err (err) => std :: panic :: panic_any (err) , } match self . dropper . join () { Ok (_) => () , Err (err) => { std :: panic :: panic_any (err) ; } } match self . writer . join () { Ok (r) => r , Err (err) => { std :: panic :: panic_any (err) ; } } } }
};
}
