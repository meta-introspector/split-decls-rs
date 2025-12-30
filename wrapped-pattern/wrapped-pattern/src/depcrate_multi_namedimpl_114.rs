// Generated macro for impl_114 (impl)
macro_rules! Depcrate_multi_namedimpl_114 {
() => {
// Module: crate::multi_named
// Provides: {"impl_114"}
// Dependencies: {}
# [cfg (feature = "litemap")] impl < 'k , K , W , S > PlaceholderValueProvider < MultiNamedPlaceholderKey < 'k > > for LiteMap < K , W , S > where K : Ord + core :: borrow :: Borrow < str > , W : Writeable , S : litemap :: store :: Store < K , W > , { type Error = MissingNamedPlaceholderError < 'k > ; type W < 'a > = Result < & 'a W , Self :: Error > where Self : 'a ; type L < 'a , 'l > = & 'l str where Self : 'a ; # [inline] fn value_for < 'a > (& 'a self , key : MultiNamedPlaceholderKey < 'k >) -> Self :: W < 'a > { match self . get (key . 0) { Some (value) => Ok (value) , None => Err (MissingNamedPlaceholderError { name : key . 0 }) , } } # [inline] fn map_literal < 'a , 'l > (& 'a self , literal : & 'l str) -> Self :: L < 'a , 'l > { literal } }
};
}
