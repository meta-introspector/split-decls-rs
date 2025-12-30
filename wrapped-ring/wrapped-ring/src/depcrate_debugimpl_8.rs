// Generated macro for impl_8 (impl)
macro_rules! Depcrate_debugimpl_8 {
() => {
// Module: crate::debug
// Provides: {"impl_8"}
// Dependencies: {}
impl core :: fmt :: Debug for HexStr < '_ > { fn fmt (& self , fmt : & mut core :: fmt :: Formatter) -> Result < () , core :: fmt :: Error > { fmt . write_str ("\"") ? ; write_hex_bytes (fmt , self . 0) ? ; fmt . write_str ("\"") ? ; Ok (()) } }
};
}
