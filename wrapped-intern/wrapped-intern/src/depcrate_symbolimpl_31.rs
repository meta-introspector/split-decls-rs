// Generated macro for impl_31 (impl)
macro_rules! Depcrate_symbolimpl_31 {
() => {
// Module: crate::symbol
// Provides: {"impl_31"}
// Dependencies: {}
impl Symbol { pub fn intern (s : & str) -> Self { let storage = MAP . get_or_init (symbols :: prefill) ; let new_symbol = Symbol { repr : TaggedArcPtr :: arc (Arc :: new (Box :: < str > :: from (s))) } ; let new_symbol_arc = Arc :: new (new_symbol) ; let entry = storage . entry (new_symbol_arc . clone ()) . or_insert (()) ; entry . key () . deref () . clone () } pub fn integer (i : usize) -> Self { match i { 0 => symbols :: INTEGER_0 , 1 => symbols :: INTEGER_1 , 2 => symbols :: INTEGER_2 , 3 => symbols :: INTEGER_3 , 4 => symbols :: INTEGER_4 , 5 => symbols :: INTEGER_5 , 6 => symbols :: INTEGER_6 , 7 => symbols :: INTEGER_7 , 8 => symbols :: INTEGER_8 , 9 => symbols :: INTEGER_9 , 10 => symbols :: INTEGER_10 , 11 => symbols :: INTEGER_11 , 12 => symbols :: INTEGER_12 , 13 => symbols :: INTEGER_13 , 14 => symbols :: INTEGER_14 , 15 => symbols :: INTEGER_15 , i => Symbol :: intern (& format ! ("{i}")) , } } pub fn empty () -> Self { symbols :: __empty } # [inline] pub fn as_str (& self) -> & str { self . repr . as_str () } # [cold] fn drop_slow (symbol_to_remove : & Symbol) { let storage = MAP . get_or_init (symbols :: prefill) ; storage . remove (symbol_to_remove) ; } }
};
}
