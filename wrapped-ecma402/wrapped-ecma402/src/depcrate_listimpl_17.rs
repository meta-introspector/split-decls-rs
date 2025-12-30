// Generated macro for impl_17 (impl)
macro_rules! Depcrate_listimpl_17 {
() => {
// Module: crate::list
// Provides: {"impl_17"}
// Dependencies: {}
impl ecma402_traits :: listformat :: Format for ListFormat { type Error = icu_provider :: DataError ; fn try_new < L > (locale : L , opts : Options) -> Result < Self , Self :: Error > where L : Locale , Self : Sized , { # [expect (clippy :: unwrap_used)] let locale = icu :: locale :: Locale :: try_from_str (& locale . to_string ()) . unwrap () ; let prefs = icu :: list :: ListFormatterPreferences :: from (& locale) ; let length = match opts . style { Style :: Long => icu :: list :: options :: ListLength :: Wide , Style :: Narrow => icu :: list :: options :: ListLength :: Narrow , Style :: Short => icu :: list :: options :: ListLength :: Short , } ; let options = icu :: list :: options :: ListFormatterOptions :: default () . with_length (length) ; Ok (Self (match opts . in_type { Type :: Conjunction => icu :: list :: ListFormatter :: try_new_and (prefs , options) , Type :: Disjunction => icu :: list :: ListFormatter :: try_new_or (prefs , options) , } ?)) } fn format < I , L , W > (& self , list : L , writer : & mut W) -> fmt :: Result where I : Display , L : IntoIterator < Item = I > , W : Write , { struct WriteableWrap < J : Display > (J) ; impl < J : Display > Writeable for WriteableWrap < J > { fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { write ! (sink , "{}" , self . 0) } } let values = list . into_iter () . map (WriteableWrap) . collect :: < Vec < _ > > () ; self . 0 . format (values . iter ()) . write_to (writer) } }
};
}
