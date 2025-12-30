// Generated macro for OVERHEAD (const)
macro_rules! DepcrateOVERHEAD {
() => {
// Module: crate
// Provides: {"OVERHEAD"}
// Dependencies: {}
const OVERHEAD : usize = match round_up_to (MALLOC_OVERHEAD + FOOTER_SIZE , CHUNK_ALIGN) { Some (x) => x , None => panic ! () , } ;
};
}
