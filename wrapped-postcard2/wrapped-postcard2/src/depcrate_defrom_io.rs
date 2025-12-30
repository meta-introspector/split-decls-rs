// Generated macro for from_io (function)
macro_rules! Depcrate_defrom_io {
() => {
// Module: crate::de
// Provides: {"from_io"}
// Dependencies: {}
# [doc = " Deserialize a message of type `T` from a [`std::io::Read`]."] # [cfg (feature = "use-std")] pub fn from_io < 'a , T , R > (val : (R , & 'a mut [u8])) -> Result < (T , (R , & 'a mut [u8])) > where T : Deserialize < 'a > , R : std :: io :: Read + 'a , { let flavor = flavors :: io :: io :: IOReader :: new (val . 0 , val . 1) ; let mut deserializer = Deserializer :: from_flavor (flavor) ; let t = T :: deserialize (& mut deserializer) ? ; Ok ((t , deserializer . finalize () ?)) }
};
}
