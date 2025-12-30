// Generated macro for impl_from_tuples_for_inmemorydir (macro)
macro_rules! Depcrate_compareimpl_from_tuples_for_inmemorydir {
() => {
// Module: crate::compare
// Provides: {"impl_from_tuples_for_inmemorydir"}
// Dependencies: {}
# [doc = " Extend `impl_from_tuple_for_inmemorydir` to generate for the specified tuple and all smaller"] # [doc = " tuples"] macro_rules ! impl_from_tuples_for_inmemorydir { ($ var1 : ident $ path1 : ident $ data1 : ident , $ ($ var : ident $ path : ident $ data : ident) ,+) => { impl_from_tuples_for_inmemorydir ! (__impl $ var1 $ path1 $ data1 ; $ ($ var $ path $ data) ,+) ; } ; (__impl $ ($ var : ident $ path : ident $ data : ident) ,+; $ var1 : ident $ path1 : ident $ data1 : ident $ (,$ var2 : ident $ path2 : ident $ data2 : ident) *) => { impl_from_tuple_for_inmemorydir ! ($ ($ var $ path $ data) ,+) ; impl_from_tuples_for_inmemorydir ! (__impl $ ($ var $ path $ data) ,+, $ var1 $ path1 $ data1 ; $ ($ var2 $ path2 $ data2) ,*) ; } ; (__impl $ ($ var : ident $ path : ident $ data : ident) ,+;) => { impl_from_tuple_for_inmemorydir ! ($ ($ var $ path $ data) ,+) ; } }
};
}
