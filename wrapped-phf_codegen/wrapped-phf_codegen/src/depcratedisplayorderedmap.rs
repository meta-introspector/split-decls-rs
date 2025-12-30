// Generated macro for DisplayOrderedMap (struct)
macro_rules! DepcrateDisplayOrderedMap {
() => {
// Module: crate
// Provides: {"DisplayOrderedMap"}
// Dependencies: {}
# [doc = " An adapter for printing a [`OrderedMap`](OrderedMap)."] pub struct DisplayOrderedMap < 'a , K > { path : & 'a str , state : HashState , keys : & 'a [K] , values : & 'a [Cow < 'a , str >] , }
};
}
