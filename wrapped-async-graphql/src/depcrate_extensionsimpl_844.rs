// Generated macro for impl_844 (impl)
macro_rules! Depcrate_extensionsimpl_844 {
() => {
// Module: crate::extensions
// Provides: {"impl_844"}
// Dependencies: {}
impl NextResolve < '_ > { # [doc = " Call the [Extension::resolve] function of next extension."] pub async fn run (self , ctx : & ExtensionContext < '_ > , info : ResolveInfo < '_ > ,) -> ServerResult < Option < Value > > { if let Some ((first , next)) = self . chain . split_first () { first . resolve (ctx , info , NextResolve { chain : next , resolve_fut : self . resolve_fut , } ,) . await } else { self . resolve_fut . await } } }
};
}
