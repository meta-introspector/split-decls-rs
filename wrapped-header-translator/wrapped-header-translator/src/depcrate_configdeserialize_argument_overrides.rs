// Generated macro for deserialize_argument_overrides (function)
macro_rules! Depcrate_configdeserialize_argument_overrides {
() => {
// Module: crate::config
// Provides: {"deserialize_argument_overrides"}
// Dependencies: {}
fn deserialize_argument_overrides < 'de , D > (deserializer : D ,) -> Result < HashMap < usize , TypeOverride > , D :: Error > where D : de :: Deserializer < 'de > , { let str_map = HashMap :: < Cow < '_ , str > , TypeOverride > :: deserialize (deserializer) ? ; let original_len = str_map . len () ; let data = { str_map . into_iter () . map (| (str_key , value) | match str_key . parse () { Ok (int_key) => Ok ((int_key , value)) , Err (_) => Err ({ de :: Error :: invalid_value (de :: Unexpected :: Str (& str_key) , & "a non-negative integer" ,) }) , }) . collect :: < Result < HashMap < _ , _ > , _ > > () ? } ; if data . len () < original_len { return Err (de :: Error :: custom ("duplicate integer key")) ; } Ok (data) }
};
}
