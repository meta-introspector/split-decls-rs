// Generated macro for impl_1233 (impl)
macro_rules! Depcrate_util_rangeintimpl_1233 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1233"}
// Dependencies: {}
impl < const MIN : i128 , const MAX : i128 > core :: fmt :: Debug for RangedDebug < MIN , MAX > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { # [cfg (not (debug_assertions))] { let val = self . rint . get_unchecked () ; if < ri128 < MIN , MAX > > :: contains (val) { val . fmt (f) } else { write ! (f , "#{val:?} [out of range: {MIN}..={MAX}]#") } } # [cfg (debug_assertions)] { let val = self . rint . get_unchecked () ; let min = self . rint . min ; let max = self . rint . max ; if < ri128 < MIN , MAX > > :: contains (val) && < ri128 < MIN , MAX > > :: contains (min) && < ri128 < MIN , MAX > > :: contains (max) { val . fmt (f) } else { write ! (f , "#{val:?} \
                     [out of range: {MIN}..={MAX}] \
                     [possible range: {min}..={max}]#" ,) } } } }
};
}
