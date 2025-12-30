// Generated macro for impl_378 (impl)
macro_rules! Depcrate_providerimpl_378 {
() => {
// Module: crate::provider
// Provides: {"impl_378"}
// Dependencies: {}
impl < 'data > PropertyUnicodeSet < 'data > { # [inline] pub (crate) fn contains_str (& self , s : & str) -> bool { match * self { Self :: CPInversionListStrList (ref l) => l . contains_str (s) , } } # [inline] pub (crate) fn contains32 (& self , cp : u32) -> bool { match * self { Self :: CPInversionListStrList (ref l) => l . contains32 (cp) , } } # [inline] pub (crate) fn contains (& self , ch : char) -> bool { match * self { Self :: CPInversionListStrList (ref l) => l . contains (ch) , } } # [inline] pub (crate) fn from_code_point_inversion_list_string_list (l : CodePointInversionListAndStringList < 'static > ,) -> Self { Self :: CPInversionListStrList (l) } # [inline] pub (crate) fn as_code_point_inversion_list_string_list (& '_ self ,) -> Option < & '_ CodePointInversionListAndStringList < 'data > > { match * self { Self :: CPInversionListStrList (ref l) => Some (l) , } } # [inline] pub (crate) fn to_code_point_inversion_list_string_list (& self ,) -> CodePointInversionListAndStringList < '_ > { match * self { Self :: CPInversionListStrList (ref t) => ZeroFrom :: zero_from (t) , } } }
};
}
