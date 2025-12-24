use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Hashes strings > 16 bytes.
///
/// # Safety
/// v.len() must be > 16 bytes.
#[cold]
#[inline(never)]
unsafe fn hash_bytes_long(mut v: &[u8], accumulator: u64, seeds: &[u64; 6]) -> u64 {
    let mut s0 = accumulator;
    let mut s1 = s0.wrapping_add(seeds[1]);
    if v.len() > 128 {
        cold_path();
        let mut s2 = s0.wrapping_add(seeds[2]);
        let mut s3 = s0.wrapping_add(seeds[3]);
        if v.len() > 256 {
            cold_path();
            let mut s4 = s0.wrapping_add(seeds[4]);
            let mut s5 = s0.wrapping_add(seeds[5]);
            loop {
                unsafe {
                    s0 = folded_multiply(load(v, 0) ^ s0, load(v, 48) ^ seeds[0]);
                    s1 = folded_multiply(load(v, 8) ^ s1, load(v, 56) ^ seeds[0]);
                    s2 = folded_multiply(load(v, 16) ^ s2, load(v, 64) ^ seeds[0]);
                    s3 = folded_multiply(load(v, 24) ^ s3, load(v, 72) ^ seeds[0]);
                    s4 = folded_multiply(load(v, 32) ^ s4, load(v, 80) ^ seeds[0]);
                    s5 = folded_multiply(load(v, 40) ^ s5, load(v, 88) ^ seeds[0]);
                }
                v = &v[96..];
                if v.len() <= 256 {
                    break;
                }
            }
            s0 ^= s4;
            s1 ^= s5;
        }
        loop {
            unsafe {
                s0 = folded_multiply(load(v, 0) ^ s0, load(v, 32) ^ seeds[0]);
                s1 = folded_multiply(load(v, 8) ^ s1, load(v, 40) ^ seeds[0]);
                s2 = folded_multiply(load(v, 16) ^ s2, load(v, 48) ^ seeds[0]);
                s3 = folded_multiply(load(v, 24) ^ s3, load(v, 56) ^ seeds[0]);
            }
            v = &v[64..];
            if v.len() <= 128 {
                break;
            }
        }
        s0 ^= s2;
        s1 ^= s3;
    }
    let len = v.len();
    unsafe {
        s0 = folded_multiply(load(v, 0) ^ s0, load(v, len - 16) ^ seeds[0]);
        s1 = folded_multiply(load(v, 8) ^ s1, load(v, len - 8) ^ seeds[0]);
        if len >= 32 {
            s0 = folded_multiply(load(v, 16) ^ s0, load(v, len - 32) ^ seeds[0]);
            s1 = folded_multiply(load(v, 24) ^ s1, load(v, len - 24) ^ seeds[0]);
            if len >= 64 {
                s0 = folded_multiply(load(v, 32) ^ s0, load(v, len - 48) ^ seeds[0]);
                s1 = folded_multiply(load(v, 40) ^ s1, load(v, len - 40) ^ seeds[0]);
                if len >= 96 {
                    s0 = folded_multiply(load(v, 48) ^ s0, load(v, len - 64) ^ seeds[0]);
                    s1 = folded_multiply(load(v, 56) ^ s1, load(v, len - 56) ^ seeds[0]);
                }
            }
        }
    }
    s0 ^ s1
}
