// Generated macro for huge_encode (function)
macro_rules! Depcrate_punycodehuge_encode {
() => {
// Module: crate::punycode
// Provides: {"huge_encode"}
// Dependencies: {}
# [test] # [ignore = "slow"] # [cfg (target_pointer_width = "64")] fn huge_encode () { let mut buf = String :: new () ; assert ! (encode_into ::< _ , _ , ExternalCaller > (core :: iter :: repeat ('ß') . take (u32 :: MAX as usize + 1) , & mut buf) . is_err ()) ; assert_eq ! (buf . len () , 0) ; }
};
}
