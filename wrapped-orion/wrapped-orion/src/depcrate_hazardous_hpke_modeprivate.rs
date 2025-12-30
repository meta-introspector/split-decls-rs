// Generated macro for private (module)
macro_rules! Depcrate_hazardous_hpke_modeprivate {
() => {
// Module: crate::hazardous::hpke::mode
// Provides: {"private"}
// Dependencies: {}
pub (crate) mod private { use crate :: errors :: UnknownCryptoError ; # [doc = " Marker trait intended for a suite that implements this HPKE mode."] pub trait Base { } # [doc = " Marker trait intended for a suite that implements this HPKE mode."] pub trait Psk { } # [doc = " Marker trait intended for a suite that implements this HPKE mode."] pub trait Auth { } # [doc = " Marker trait intended for a suite that implements this HPKE mode."] pub trait AuthPsk { } # [repr (u8)] # [doc = " HPKE modes utility."] pub enum HpkeMode { # [doc = " Base mode."] Base = 0x00u8 , # [doc = " PSK mode."] Psk = 0x01u8 , # [doc = " Auth mode."] Auth = 0x02u8 , # [doc = " Auth+PSK mode."] AuthPsk = 0x03u8 , } impl HpkeMode { pub (crate) fn verify_psk_inputs (& self , psk : & [u8] , psk_id : & [u8] ,) -> Result < () , UnknownCryptoError > { match * self { HpkeMode :: Base | HpkeMode :: Auth => { match (psk . is_empty () , psk_id . is_empty ()) { (true , true) => Ok (()) , (_ , _) => Err (UnknownCryptoError) , } } HpkeMode :: Psk | HpkeMode :: AuthPsk => { match (psk . is_empty () , psk_id . is_empty ()) { (false , false) => Ok (()) , (_ , _) => Err (UnknownCryptoError) , } } } } # [doc = " Returns the `mode_id` for this HPKE mode."] pub fn mode_id (& self) -> u8 { match self { Self :: Base => 0x00u8 , Self :: Psk => 0x01u8 , Self :: Auth => 0x02u8 , Self :: AuthPsk => 0x03u8 , } } } }
};
}
