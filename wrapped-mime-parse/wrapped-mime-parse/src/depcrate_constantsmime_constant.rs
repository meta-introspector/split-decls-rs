// Generated macro for mime_constant (macro)
macro_rules! Depcrate_constantsmime_constant {
() => {
// Module: crate::constants
// Provides: {"mime_constant"}
// Dependencies: {}
macro_rules ! mime_constant { ($ id : ident , $ src : expr , $ slash : expr) => (mime_constant ! ($ id , $ src , $ slash , None) ;) ; ($ id : ident , $ src : expr , $ slash : expr , $ plus : expr) => (mime_constant ! (FULL $ id , $ src , $ slash , $ plus , ParamSource :: None) ;) ; ($ id : ident , $ src : expr , $ slash : expr , $ plus : expr , $ params : expr) => (mime_constant ! (FULL $ id , $ src , $ slash , $ plus , ParamSource :: Utf8 ($ params)) ;) ; (FULL $ id : ident , $ src : expr , $ slash : expr , $ plus : expr , $ params : expr) => (impl Atoms { const $ id : Source = Source :: Atom (__Atoms ::$ id as u8 , $ src) ; } # [doc = "`"] # [doc = $ src] # [doc = "`"] pub const $ id : Mime = Mime { source : Atoms ::$ id , slash : $ slash , plus : $ plus , params : $ params , } ;) }
};
}
