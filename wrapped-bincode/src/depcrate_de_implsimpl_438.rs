// Generated macro for impl_438 (impl)
macro_rules! Depcrate_de_implsimpl_438 {
() => {
// Module: crate::de::impls
// Provides: {"impl_438"}
// Dependencies: {}
impl < Context > Decode < Context > for Duration { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { const NANOS_PER_SEC : u64 = 1_000_000_000 ; let secs : u64 = Decode :: decode (decoder) ? ; let nanos : u32 = Decode :: decode (decoder) ? ; if secs . checked_add (u64 :: from (nanos) / NANOS_PER_SEC) . is_none () { return Err (DecodeError :: InvalidDuration { secs , nanos }) ; } Ok (Duration :: new (secs , nanos)) } }
};
}
