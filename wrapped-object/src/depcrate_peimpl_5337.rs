// Generated macro for impl_5337 (impl)
macro_rules! Depcrate_peimpl_5337 {
() => {
// Module: crate::pe
// Provides: {"impl_5337"}
// Dependencies: {}
impl Guid { # [inline] pub fn data1 (self) -> U32 < LE > { U32 :: from_bytes (self . 0 [0 .. 4] . try_into () . unwrap ()) } # [inline] pub fn data2 (self) -> U16 < LE > { U16 :: from_bytes (self . 0 [4 .. 6] . try_into () . unwrap ()) } # [inline] pub fn data3 (self) -> U16 < LE > { U16 :: from_bytes (self . 0 [6 .. 8] . try_into () . unwrap ()) } # [inline] pub fn data4 (self) -> [u8 ; 8] { self . 0 [8 .. 16] . try_into () . unwrap () } }
};
}
