// Generated macro for Fat (struct)
macro_rules! Depcrate_packed_teddy_genericFat {
() => {
// Module: crate::packed::teddy::generic
// Provides: {"Fat"}
// Dependencies: {}
# [doc = " A \"fat\" Teddy implementation that is generic over both the vector type"] # [doc = " and the minimum length of the patterns being searched for."] # [doc = ""] # [doc = " Only 1, 2, 3 and 4 bytes are supported as minimum lengths."] # [derive (Clone , Debug)] pub (crate) struct Fat < V , const BYTES : usize > { # [doc = " A generic data structure for doing \"fat\" Teddy verification."] teddy : Teddy < 16 > , # [doc = " The masks used as inputs to the shuffle operation to generate"] # [doc = " candidates (which are fed into the verification routines)."] masks : [Mask < V > ; BYTES] , }
};
}
