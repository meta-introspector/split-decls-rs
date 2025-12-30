// Generated macro for FromHexIter (struct)
macro_rules! DepcrateFromHexIter {
() => {
// Module: crate
// Provides: {"FromHexIter"}
// Dependencies: {}
# [doc = " An iterator decoding hex-encoded characters into bytes."] pub struct FromHexIter < 'a > { err : bool , inner : & 'a str , iter : iter :: Enumerate < core :: str :: Bytes < 'a > > , }
};
}
