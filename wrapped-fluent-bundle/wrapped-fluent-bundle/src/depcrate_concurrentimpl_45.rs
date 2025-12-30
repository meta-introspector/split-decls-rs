// Generated macro for impl_45 (impl)
macro_rules! Depcrate_concurrentimpl_45 {
() => {
// Module: crate::concurrent
// Provides: {"impl_45"}
// Dependencies: {}
impl MemoizerKind for IntlLangMemoizer { fn new (lang : LanguageIdentifier) -> Self where Self : Sized , { Self :: new (lang) } fn with_try_get_threadsafe < I , R , U > (& self , args : I :: Args , cb : U) -> Result < R , I :: Error > where Self : Sized , I : Memoizable + Send + Sync + 'static , I :: Args : Send + Sync + 'static , U : FnOnce (& I) -> R , { self . with_try_get (args , cb) } fn stringify_value (& self , value : & dyn FluentType) -> std :: borrow :: Cow < 'static , str > { value . as_string_threadsafe (self) } }
};
}
