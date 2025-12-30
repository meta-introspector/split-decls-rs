// Generated macro for DisplayMap (struct)
macro_rules! DepcrateDisplayMap {
() => {
// Module: crate
// Provides: {"DisplayMap"}
// Dependencies: {}
# [doc = " An adapter for printing a [`Map`](Map)."] pub struct DisplayMap < 'a , K > { path : & 'a str , state : HashState , keys : & 'a [K] , values : & 'a [Cow < 'a , str >] , }
};
}
