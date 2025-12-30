// Generated macro for tests (module)
macro_rules! Depcrate_ots_publictests {
() => {
// Module: crate::ots::public
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: constants :: ID_LEN ; use crate :: error :: LmsDeserializeError ; use crate :: ots :: modes :: { LmsOtsSha256N32W4 , LmsOtsSha256N32W8 } ; use crate :: ots :: private :: SigningKey ; use crate :: ots :: public :: VerifyingKey ; use hybrid_array :: Array ; use rand :: rng ; # [test] fn test_serde () { let pk = SigningKey :: < LmsOtsSha256N32W8 > :: new (0 , [0xbb ; ID_LEN] , & mut rng ()) . public () ; let pk_serialized : Array < u8 , _ > = pk . clone () . into () ; let bytes = pk_serialized . as_slice () ; let pk_deserialized = VerifyingKey :: < LmsOtsSha256N32W8 > :: try_from (bytes) ; assert ! (pk_deserialized . is_ok ()) ; let pk_deserialized = pk_deserialized . unwrap () ; assert_eq ! (pk , pk_deserialized) ; let pk_wrongalgo = VerifyingKey :: < LmsOtsSha256N32W4 > :: try_from (bytes) ; let pk_short = VerifyingKey :: < LmsOtsSha256N32W8 > :: try_from (& bytes [0 .. (bytes . len () - 1)]) ; let mut long_bytes = pk_serialized . into_iter () . collect :: < Vec < _ > > () ; long_bytes . push (0) ; let pk_long = VerifyingKey :: < LmsOtsSha256N32W8 > :: try_from (long_bytes . as_slice ()) ; assert_eq ! (pk_wrongalgo , Err (LmsDeserializeError :: WrongAlgorithm)) ; assert_eq ! (pk_short , Err (LmsDeserializeError :: TooShort)) ; assert_eq ! (pk_long , Err (LmsDeserializeError :: TooLong)) ; } }
};
}
