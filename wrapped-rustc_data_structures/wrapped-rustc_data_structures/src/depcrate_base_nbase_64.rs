// Generated macro for BASE_64 (const)
macro_rules! Depcrate_base_nBASE_64 {
() => {
// Module: crate::base_n
// Provides: {"BASE_64"}
// Dependencies: {}
const BASE_64 : [ascii :: Char ; MAX_BASE] = { let bytes = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@$" ; let Some (ascii) = bytes . as_ascii () else { panic ! () } ; * ascii } ;
};
}
