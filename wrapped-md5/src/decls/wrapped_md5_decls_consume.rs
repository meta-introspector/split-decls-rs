use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[rustfmt::skip]
#[allow(clippy::double_parens, clippy::needless_range_loop)]
fn consume(Context { buffer, count, state }: &mut Context, data: &[u8]) {
    let mut input = [0u32; 16];
    let mut k = ((*count >> 3) & 0x3f) as usize;
    *count = count.wrapping_add((data.len() as u64) << 3);
    for &value in data {
        buffer[k] = value;
        k += 1;
        if k == 0x40 {
            let mut j = 0;
            for i in 0..16 {
                input[i] = ((buffer[j + 3] as u32) << 24)
                    | ((buffer[j + 2] as u32) << 16) | ((buffer[j + 1] as u32) << 8)
                    | ((buffer[j] as u32));
                j += 4;
            }
            transform(state, &input);
            k = 0;
        }
    }
}
