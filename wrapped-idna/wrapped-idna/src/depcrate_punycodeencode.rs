// Generated macro for encode (function)
macro_rules! Depcrate_punycodeencode {
() => {
// Module: crate::punycode
// Provides: {"encode"}
// Dependencies: {}
# [doc = " Convert Unicode to Punycode."] # [doc = ""] # [doc = " Return None on overflow, which can only happen on inputs that would take more than"] # [doc = " 63 encoded bytes, the DNS limit on domain name labels."] pub fn encode (input : & [char]) -> Option < String > { if input . len () > u32 :: MAX as usize { return None ; } let mut buf = String :: with_capacity (input . len ()) ; encode_into :: < _ , _ , ExternalCaller > (input . iter () . copied () , & mut buf) . ok () . map (| () | buf) }
};
}
