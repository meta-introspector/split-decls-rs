use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    use crate::decode::{hex_decode, hex_decode_with_case, CheckCase};
    use crate::encode::{hex_encode, hex_string};
    use crate::{hex_encode_upper, hex_string_upper, vectorization_support, Vectorization};
    use proptest::proptest;
    #[cfg(not(feature = "alloc"))]
    const CAPACITY: usize = 128;
    #[test]
    fn test_feature_detection() {
        let vector_support = vectorization_support();
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            match vector_support {
                Vectorization::AVX2 => assert!(is_x86_feature_detected!("avx2")),
                Vectorization::SSE41 => assert!(is_x86_feature_detected!("sse4.1")),
                Vectorization::None => {
                    assert!(
                        !cfg!(target_feature = "sse")
                            || !is_x86_feature_detected!("avx2")
                                && !is_x86_feature_detected!("sse4.1")
                    )
                }
            }
        }
        #[cfg(target_arch = "aarch64")]
        match vector_support {
            Vectorization::Neon => {
                assert!(std::arch::is_aarch64_feature_detected!("neon"))
            }
            Vectorization::None => {
                assert!(
                    !cfg!(target_feature = "neon")
                        || !std::arch::is_aarch64_feature_detected!("neon")
                )
            }
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
        assert_eq!(vector_support, Vectorization::None);
    }
    fn _test_hex_encode(s: &String) {
        let mut buffer = vec![0; s.as_bytes().len() * 2];
        {
            let encode = &*hex_encode(s.as_bytes(), &mut buffer).unwrap();
            #[cfg(feature = "alloc")]
            let hex_string = hex_string(s.as_bytes());
            #[cfg(not(feature = "alloc"))]
            let hex_string = hex_string::<CAPACITY>(s.as_bytes());
            assert_eq!(encode, hex::encode(s));
            assert_eq!(hex_string.as_str(), hex::encode(s));
        }
        {
            let encode_upper = &*hex_encode_upper(s.as_bytes(), &mut buffer).unwrap();
            #[cfg(feature = "alloc")]
            let hex_string_upper = hex_string_upper(s.as_bytes());
            #[cfg(not(feature = "alloc"))]
            let hex_string_upper = hex_string_upper::<CAPACITY>(s.as_bytes());
            assert_eq!(encode_upper, hex::encode_upper(s));
            assert_eq!(hex_string_upper.as_str(), hex::encode_upper(s));
        }
    }
    #[cfg(feature = "alloc")]
    proptest! {
        #[test] fn test_hex_encode(ref s in ".*") { _test_hex_encode(s); }
    }
    #[cfg(not(feature = "alloc"))]
    proptest! {
        #[test] fn test_hex_encode(ref s in ".{0,16}") { _test_hex_encode(s); }
    }
    fn _test_hex_decode(s: &String) {
        let len = s.as_bytes().len();
        {
            let mut dst = Vec::with_capacity(len);
            dst.resize(len, 0);
            #[cfg(feature = "alloc")]
            let hex_string = hex_string(s.as_bytes());
            #[cfg(not(feature = "alloc"))]
            let hex_string = hex_string::<CAPACITY>(s.as_bytes());
            hex_decode(hex_string.as_bytes(), &mut dst).unwrap();
            hex_decode_with_case(hex_string.as_bytes(), &mut dst, CheckCase::Lower).unwrap();
            assert_eq!(&dst[..], s.as_bytes());
        }
        {
            let mut dst = Vec::with_capacity(len);
            dst.resize(len, 0);
            #[cfg(feature = "alloc")]
            let hex_string_upper = hex_string_upper(s.as_bytes());
            #[cfg(not(feature = "alloc"))]
            let hex_string_upper = hex_string_upper::<CAPACITY>(s.as_bytes());
            hex_decode_with_case(hex_string_upper.as_bytes(), &mut dst, CheckCase::Upper).unwrap();
            assert_eq!(&dst[..], s.as_bytes());
        }
    }
    #[cfg(feature = "alloc")]
    proptest! {
        #[test] fn test_hex_decode(ref s in ".+") { _test_hex_decode(s); }
    }
    #[cfg(not(feature = "alloc"))]
    proptest! {
        #[test] fn test_hex_decode(ref s in ".{1,16}") { _test_hex_decode(s); }
    }
    fn _test_hex_decode_check(s: &String, ok: bool) {
        let len = s.as_bytes().len();
        let mut dst = Vec::with_capacity(len / 2);
        dst.resize(len / 2, 0);
        assert!(hex_decode(s.as_bytes(), &mut dst).is_ok() == ok);
    }
    proptest! {
        #[test] fn test_hex_decode_check(ref s in "([0-9a-fA-F][0-9a-fA-F])+") {
        _test_hex_decode_check(s, true); }
    }
    proptest! {
        #[test] fn test_hex_decode_check_odd(ref s in "[0-9a-fA-F]{11}") {
        _test_hex_decode_check(s, false); }
    }
}
