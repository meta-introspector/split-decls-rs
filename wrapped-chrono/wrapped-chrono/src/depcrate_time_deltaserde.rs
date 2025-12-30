// Generated macro for serde (module)
macro_rules! Depcrate_time_deltaserde {
() => {
// Module: crate::time_delta
// Provides: {"serde"}
// Dependencies: {}
# [cfg (feature = "serde")] mod serde { use super :: TimeDelta ; use serde :: { Deserialize , Deserializer , Serialize , Serializer , de :: Error } ; impl Serialize for TimeDelta { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { < (i64 , i32) as Serialize > :: serialize (& (self . secs , self . nanos) , serializer) } } impl < 'de > Deserialize < 'de > for TimeDelta { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { let (secs , nanos) = < (i64 , i32) as Deserialize > :: deserialize (deserializer) ? ; TimeDelta :: new (secs , nanos as u32) . ok_or (Error :: custom ("TimeDelta out of bounds")) } } # [cfg (test)] mod tests { use super :: { super :: MAX , TimeDelta } ; # [test] fn test_serde () { let duration = TimeDelta :: new (123 , 456) . unwrap () ; assert_eq ! (serde_json :: from_value ::< TimeDelta > (serde_json :: to_value (duration) . unwrap ()) . unwrap () , duration) ; } # [test] # [should_panic (expected = "TimeDelta out of bounds")] fn test_serde_oob_panic () { let _ = serde_json :: from_value :: < TimeDelta > (serde_json :: json ! ([MAX . secs + 1 , 0])) . unwrap () ; } } }
};
}
