// Generated macro for from_eio (function)
macro_rules! Depcrate_defrom_eio {
() => {
// Module: crate::de
// Provides: {"from_eio"}
// Dependencies: {}
# [doc = " Deserialize a message of type `T` from a [`embedded_io`](crate::eio::embedded_io)::[`Read`](crate::eio::Read)."] # [cfg (any (feature = "embedded-io-04" , feature = "embedded-io-06"))] pub fn from_eio < 'a , T , R > (val : (R , & 'a mut [u8])) -> Result < (T , (R , & 'a mut [u8])) > where T : Deserialize < 'a > , R : crate :: eio :: Read + 'a , { let flavor = flavors :: io :: eio :: EIOReader :: new (val . 0 , val . 1) ; let mut deserializer = Deserializer :: from_flavor (flavor) ; let t = T :: deserialize (& mut deserializer) ? ; Ok ((t , deserializer . finalize () ?)) }
};
}
