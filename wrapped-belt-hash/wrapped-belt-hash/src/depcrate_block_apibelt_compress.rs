// Generated macro for belt_compress (function)
macro_rules! Depcrate_block_apibelt_compress {
() => {
// Module: crate::block_api
// Provides: {"belt_compress"}
// Dependencies: {}
# [doc = " Compression function described in the section 6.3.2"] # [inline (always)] pub fn belt_compress (x1 : [u32 ; 4] , x2 : [u32 ; 4] , x34 : [u32 ; 8]) -> ([u32 ; 4] , [u32 ; 8]) { let x3 = [x34 [0] , x34 [1] , x34 [2] , x34 [3]] ; let x4 = [x34 [4] , x34 [5] , x34 [6] , x34 [7]] ; let t1 = belt_block_raw (xor (x3 , x4) , & concat (x1 , x2)) ; let s = xor (xor (t1 , x3) , x4) ; let t2 = belt_block_raw (x1 , & concat (s , x4)) ; let y1 = xor (t2 , x1) ; let t3 = belt_block_raw (x2 , & concat (s . map (| v | ! v) , x3)) ; let y2 = xor (t3 , x2) ; (s , concat (y1 , y2)) }
};
}
