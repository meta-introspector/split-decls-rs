use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn compress_subtree_wide<J: join::Join>(
    input: &[u8],
    key: &CVWords,
    chunk_counter: u64,
    flags: u8,
    platform: Platform,
    out: &mut [u8],
) -> usize {
    if input.len() <= platform.simd_degree() * CHUNK_LEN {
        return compress_chunks_parallel(input, key, chunk_counter, flags, platform, out);
    }
    debug_assert_eq!(platform.simd_degree().count_ones(), 1, "power of 2");
    let (left, right) = input.split_at(hazmat::left_subtree_len(input.len() as u64) as usize);
    let right_chunk_counter = chunk_counter + (left.len() / CHUNK_LEN) as u64;
    let mut cv_array = [0; 2 * MAX_SIMD_DEGREE_OR_2 * OUT_LEN];
    let degree = if left.len() == CHUNK_LEN {
        debug_assert_eq!(platform.simd_degree(), 1);
        1
    } else {
        cmp::max(platform.simd_degree(), 2)
    };
    let (left_out, right_out) = cv_array.split_at_mut(degree * OUT_LEN);
    let (left_n, right_n) = J::join(
        || compress_subtree_wide::<J>(left, key, chunk_counter, flags, platform, left_out),
        || compress_subtree_wide::<J>(right, key, right_chunk_counter, flags, platform, right_out),
    );
    debug_assert_eq!(left_n, degree);
    debug_assert!(right_n >= 1 && right_n <= left_n);
    if left_n == 1 {
        out[..2 * OUT_LEN].copy_from_slice(&cv_array[..2 * OUT_LEN]);
        return 2;
    }
    let num_children = left_n + right_n;
    compress_parents_parallel(
        &cv_array[..num_children * OUT_LEN],
        key,
        flags,
        platform,
        out,
    )
}
