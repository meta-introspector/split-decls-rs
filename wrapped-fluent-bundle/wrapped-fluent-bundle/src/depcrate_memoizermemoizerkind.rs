// Generated macro for MemoizerKind (trait)
macro_rules! Depcrate_memoizerMemoizerKind {
() => {
// Module: crate::memoizer
// Provides: {"MemoizerKind"}
// Dependencies: {}
# [doc = " This trait contains thread-safe methods which extend [`intl_memoizer::IntlLangMemoizer`]."] # [doc = " It is used as the generic bound in this crate when a memoizer is needed."] pub trait MemoizerKind : 'static { fn new (lang : LanguageIdentifier) -> Self where Self : Sized ; # [doc = " A threadsafe variant of `with_try_get` from [`intl_memoizer::IntlLangMemoizer`]."] # [doc = " The generics enforce that `Self` and its arguments are actually threadsafe."] # [doc = ""] # [doc = " `I` - The [Memoizable](intl_memoizer::Memoizable) internationalization formatter."] # [doc = ""] # [doc = " `R` - The result from the format operation."] # [doc = ""] # [doc = " `U` - The callback that accepts the instance of the intl formatter, and generates"] # [doc = "       some kind of results `R`."] fn with_try_get_threadsafe < I , R , U > (& self , args : I :: Args , callback : U) -> Result < R , I :: Error > where Self : Sized , I : Memoizable + Send + Sync + 'static , I :: Args : Send + Sync + 'static , U : FnOnce (& I) -> R ; # [doc = " Wires up the `as_string` or `as_string_threadsafe` variants for [`FluentType`]."] fn stringify_value (& self , value : & dyn FluentType) -> std :: borrow :: Cow < 'static , str > ; }
};
}
