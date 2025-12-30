// Generated macro for Access (trait)
macro_rules! Depcrate_keysAccess {
() => {
// Module: crate::keys
// Provides: {"Access"}
// Dependencies: {}
# [doc = " Safe read access to the _MALLCTL NAMESPACE_."] pub trait Access < T > { # [doc = " Read the key at `self`."] fn read (& self) -> Result < T > ; # [doc = " Write `value` at the key `self`."] fn write (& self , value : T) -> Result < () > ; # [doc = " Write `value` at the key `self` returning its previous value."] fn update (& self , value : T) -> Result < T > ; }
};
}
