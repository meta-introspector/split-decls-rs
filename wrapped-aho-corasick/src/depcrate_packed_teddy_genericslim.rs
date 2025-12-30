// Generated macro for Slim (struct)
macro_rules! Depcrate_packed_teddy_genericSlim {
() => {
// Module: crate::packed::teddy::generic
// Provides: {"Slim"}
// Dependencies: {}
# [doc = " A \"slim\" Teddy implementation that is generic over both the vector type"] # [doc = " and the minimum length of the patterns being searched for."] # [doc = ""] # [doc = " Only 1, 2, 3 and 4 bytes are supported as minimum lengths."] # [derive (Clone , Debug)] pub (crate) struct Slim < V , const BYTES : usize > { # [doc = " A generic data structure for doing \"slim\" Teddy verification."] teddy : Teddy < 8 > , # [doc = " The masks used as inputs to the shuffle operation to generate"] # [doc = " candidates (which are fed into the verification routines)."] masks : [Mask < V > ; BYTES] , }
};
}
