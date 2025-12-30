// Generated macro for _impl (module)
macro_rules! Depcrate_color_impl {
() => {
// Module: crate::color
// Provides: {"_impl"}
// Dependencies: {}
# [cfg (feature = "ansiterm")] mod _impl { use ansiterm :: { ANSIGenericString , Style } ; pub struct Brush { may_paint : bool , style : Option < Style > , } impl Brush { pub fn new (colored : bool) -> Self { Brush { may_paint : colored , style : None , } } pub fn style (& mut self , style : Style) -> & mut Self { self . style = Some (style) ; self } # [must_use] pub fn paint < 'a , I , S : 'a + ToOwned + ? Sized > (& mut self , input : I ,) -> ANSIGenericString < 'a , S > where I : Into < std :: borrow :: Cow < 'a , S > > , < S as ToOwned > :: Owned : std :: fmt :: Debug , { match (self . may_paint , self . style . as_ref () . take ()) { (true , Some (style)) => style . paint (input) , (_ , Some (_)) | (_ , None) => ANSIGenericString :: from (input) , } } } }
};
}
