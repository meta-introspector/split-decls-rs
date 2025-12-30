// Generated macro for impl_36 (impl)
macro_rules! Depcrate_bundleimpl_36 {
() => {
// Module: crate::bundle
// Provides: {"impl_36"}
// Dependencies: {}
impl crate :: memoizer :: MemoizerKind for IntlLangMemoizer { fn new (lang : LanguageIdentifier) -> Self where Self : Sized , { Self :: new (lang) } fn with_try_get_threadsafe < I , R , U > (& self , args : I :: Args , cb : U) -> Result < R , I :: Error > where Self : Sized , I : intl_memoizer :: Memoizable + Send + Sync + 'static , I :: Args : Send + Sync + 'static , U : FnOnce (& I) -> R , { self . with_try_get (args , cb) } fn stringify_value (& self , value : & dyn crate :: types :: FluentType ,) -> std :: borrow :: Cow < 'static , str > { value . as_string (self) } }
};
}
