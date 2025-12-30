// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl Write for & mut [u8] { type Error = OutOfSpace ; # [inline] fn write_all (& mut self , data : & [u8]) -> Result < () , Self :: Error > { if data . len () > self . len () { return Err (OutOfSpace (())) ; } let (prefix , suffix) = core :: mem :: take (self) . split_at_mut (data . len ()) ; prefix . copy_from_slice (data) ; * self = suffix ; Ok (()) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
