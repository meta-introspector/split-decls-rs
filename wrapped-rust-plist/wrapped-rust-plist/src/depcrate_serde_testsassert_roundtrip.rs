// Generated macro for assert_roundtrip (function)
macro_rules! Depcrate_serde_testsassert_roundtrip {
() => {
// Module: crate::serde_tests
// Provides: {"assert_roundtrip"}
// Dependencies: {}
fn assert_roundtrip < T > (obj : T , expected_events : & [Event] , roundtrip_value : bool) where T : Debug + DeserializeOwned + PartialEq + Serialize , { let mut se = new_serializer () ; obj . serialize (& mut se) . unwrap () ; let events = se . into_inner () . into_inner () ; let value = if roundtrip_value { to_value (& obj) . expect ("failed to convert object into value") } else { Value :: Boolean (false) } ; assert_eq ! (& events [..] , expected_events) ; if roundtrip_value { let expected_value = Value :: from_events (expected_events . iter () . cloned () . map (Ok)) . expect ("failed to convert expected events into value") ; assert_eq ! (value , expected_value) ; } let mut de = new_deserializer (events) ; let obj_events_roundtrip = T :: deserialize (& mut de) . unwrap () ; assert_eq ! (obj_events_roundtrip , obj) ; if roundtrip_value { let obj_value_roundtrip : T = from_value (& value) . unwrap () ; assert_eq ! (obj_value_roundtrip , obj) ; } }
};
}
