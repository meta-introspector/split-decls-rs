// Generated macro for write_fmt (function)
macro_rules! Depcrate_stripwrite_fmt {
() => {
// Module: crate::strip
// Provides: {"write_fmt"}
// Dependencies: {}
fn write_fmt (raw : & mut dyn std :: io :: Write , state : & mut StripBytes , args : std :: fmt :: Arguments < '_ > ,) -> std :: io :: Result < () > { let write_all = | buf : & [u8] | write_all (raw , state , buf) ; crate :: fmt :: Adapter :: new (write_all) . write_fmt (args) }
};
}
