// Generated macro for impl_110 (impl)
macro_rules! Depcrate_ser_flavorsimpl_110 {
() => {
// Module: crate::ser::flavors
// Provides: {"impl_110"}
// Dependencies: {}
impl < B > Cobs < B > where B : Flavor + IndexMut < usize , Output = u8 > , { # [doc = " Create a new Cobs modifier Flavor. If there is insufficient space"] # [doc = " to push the leading header byte, the method will return an Error"] pub fn try_new (mut bee : B) -> Result < Self > { bee . try_push (0) . map_err (| _ | Error :: SerializeBufferFull) ? ; Ok (Self { flav : bee , cobs : EncoderState :: default () , }) } }
};
}
