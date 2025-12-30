// Generated macro for impl_2766 (impl)
macro_rules! Depcrate_settingsimpl_2766 {
() => {
// Module: crate::settings
// Provides: {"impl_2766"}
// Dependencies: {}
impl Configurable for Builder { fn enable (& mut self , name : & str) -> SetResult < () > { use self :: detail :: Detail ; let (offset , detail) = self . lookup (name) ? ; match detail { Detail :: Bool { bit } => { self . set_bit (offset , bit , true) ; Ok (()) } Detail :: Preset => { self . apply_preset (& self . template . presets [offset ..]) ; Ok (()) } _ => Err (SetError :: BadType) , } } fn set (& mut self , name : & str , value : & str) -> SetResult < () > { use self :: detail :: Detail ; let (offset , detail) = self . lookup (name) ? ; match detail { Detail :: Bool { bit } => { self . set_bit (offset , bit , parse_bool_value (value) ?) ; } Detail :: Num => { self . bytes [offset] = value . parse () . map_err (| _ | SetError :: BadValue ("number" . to_string ())) ? ; } Detail :: Enum { last , enumerators } => { self . bytes [offset] = parse_enum_value (value , self . template . enums (last , enumerators)) ? ; } Detail :: Preset => return Err (SetError :: BadName (name . to_string ())) , } Ok (()) } }
};
}
