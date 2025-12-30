// Generated macro for tests (module)
macro_rules! Depcrate_checkedtests {
() => {
// Module: crate::checked
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "serde"))] # [allow (clippy :: unwrap_used)] mod tests { use crate :: { Checked , U64 } ; use subtle :: { Choice , ConstantTimeEq , CtOption } ; # [test] fn serde () { let test = Checked :: new (U64 :: from_u64 (0x0011223344556677)) ; let serialized = bincode :: serde :: encode_to_vec (test , bincode :: config :: standard ()) . unwrap () ; let deserialized : Checked < U64 > = bincode :: serde :: decode_from_slice (& serialized , bincode :: config :: standard ()) . unwrap () . 0 ; assert ! (bool :: from (test . ct_eq (& deserialized))) ; let test = Checked :: new (U64 :: ZERO) - Checked :: new (U64 :: ONE) ; assert ! (bool :: from (test . ct_eq (& Checked (CtOption :: new (U64 :: ZERO , Choice :: from (0)))))) ; let serialized = bincode :: serde :: encode_to_vec (test , bincode :: config :: standard ()) . unwrap () ; let deserialized : Checked < U64 > = bincode :: serde :: decode_from_slice (& serialized , bincode :: config :: standard ()) . unwrap () . 0 ; assert ! (bool :: from (test . ct_eq (& deserialized))) ; } }
};
}
