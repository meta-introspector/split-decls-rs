// Generated macro for impl_113 (impl)
macro_rules! Depcrate_multi_namedimpl_113 {
() => {
// Module: crate::multi_named
// Provides: {"impl_113"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'k , K , W > PlaceholderValueProvider < MultiNamedPlaceholderKey < 'k > > for BTreeMap < K , W > where K : Ord + core :: borrow :: Borrow < str > , W : Writeable , { type Error = MissingNamedPlaceholderError < 'k > ; type W < 'a > = Result < & 'a W , Self :: Error > where Self : 'a ; type L < 'a , 'l > = & 'l str where Self : 'a ; # [inline] fn value_for < 'a > (& 'a self , key : MultiNamedPlaceholderKey < 'k >) -> Self :: W < 'a > { match self . get (key . 0) { Some (value) => Ok (value) , None => Err (MissingNamedPlaceholderError { name : key . 0 }) , } } # [inline] fn map_literal < 'a , 'l > (& 'a self , literal : & 'l str) -> Self :: L < 'a , 'l > { literal } }
};
}
