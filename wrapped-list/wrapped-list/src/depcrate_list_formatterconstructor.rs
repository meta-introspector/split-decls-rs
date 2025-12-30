// Generated macro for constructor (macro)
macro_rules! Depcrate_list_formatterconstructor {
() => {
// Module: crate::list_formatter
// Provides: {"constructor"}
// Dependencies: {}
macro_rules ! constructor { ($ name : ident , $ name_buffer : ident , $ name_unstable : ident , $ marker : ty , $ doc : literal) => { icu_provider :: gen_buffer_data_constructors ! ((prefs : ListFormatterPreferences , options : ListFormatterOptions) -> error : DataError , # [doc = concat ! ("Creates a new [`ListFormatter`] that produces a " , $ doc , "-type list using compiled data.")] # [doc = ""] # [doc = " See the [CLDR spec](https://unicode.org/reports/tr35/tr35-general.html#ListPatterns) for"] # [doc = " an explanation of the different types."] functions : [$ name , $ name_buffer , $ name_unstable , Self]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self ::$ name)] pub fn $ name_unstable (provider : & (impl DataProvider <$ marker > + ? Sized) , prefs : ListFormatterPreferences , options : ListFormatterOptions ,) -> Result < Self , DataError > { let length = match options . length . unwrap_or_default () { ListLength :: Narrow => ListFormatterPatterns :: NARROW , ListLength :: Short => ListFormatterPatterns :: SHORT , ListLength :: Wide => ListFormatterPatterns :: WIDE , } ; let locale = <$ marker >:: make_locale (prefs . locale_preferences) ; let data = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes_and_locale (length , & locale) , .. Default :: default () }) ? . payload . cast () ; Ok (Self { data }) } } ; }
};
}
