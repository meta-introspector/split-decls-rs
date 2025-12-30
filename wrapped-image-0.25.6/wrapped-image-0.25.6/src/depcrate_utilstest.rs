// Generated macro for test (module)
macro_rules! Depcrate_utilstest {
() => {
// Module: crate::utils
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [test] fn gray_to_luma8_skip () { let check = | bit_depth , w , from , to | { assert_eq ! (super :: expand_bits (bit_depth , w , from) , to) ; } ; check (1 , 10 , & [0b11110000 , 0b11000000 , 0b00001111 , 0b11000000] , vec ! [255 , 255 , 255 , 255 , 0 , 0 , 0 , 0 , 255 , 255 , 0 , 0 , 0 , 0 , 255 , 255 , 255 , 255 , 255 , 255 ,] ,) ; check (2 , 5 , & [0b11110000 , 0b11000000 , 0b00001111 , 0b11000000] , vec ! [255 , 255 , 0 , 0 , 255 , 0 , 0 , 255 , 255 , 255] ,) ; check (2 , 4 , & [0b11110000 , 0b00001111] , vec ! [255 , 255 , 0 , 0 , 0 , 0 , 255 , 255] ,) ; check (4 , 1 , & [0b11110011 , 0b00001100] , vec ! [255 , 0]) ; } }
};
}
