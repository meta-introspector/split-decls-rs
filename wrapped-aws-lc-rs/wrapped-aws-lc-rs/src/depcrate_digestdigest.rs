// Generated macro for digest (function)
macro_rules! Depcrate_digestdigest {
() => {
// Module: crate::digest
// Provides: {"digest"}
// Dependencies: {}
# [doc = " Returns the digest of `data` using the given digest algorithm."] # [doc = ""] # [doc = " # Examples:"] # [doc = ""] # [doc = " ```"] # [doc = " # {"] # [doc = " use aws_lc_rs::{digest, test};"] # [doc = " let expected_hex = \"09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b\";"] # [doc = " let expected: Vec<u8> = test::from_hex(expected_hex).unwrap();"] # [doc = " let actual = digest::digest(&digest::SHA256, b\"hello, world\");"] # [doc = ""] # [doc = " assert_eq!(&expected, &actual.as_ref());"] # [doc = " # }"] # [doc = " ```"] # [inline] # [must_use] pub fn digest (algorithm : & 'static Algorithm , data : & [u8]) -> Digest { let mut output = [0u8 ; MAX_OUTPUT_LEN] ; (algorithm . one_shot_hash) (data , & mut output) ; Digest { algorithm , message : output , len : algorithm . output_len , } }
};
}
