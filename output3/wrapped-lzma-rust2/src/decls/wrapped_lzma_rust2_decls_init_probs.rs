use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[inline(always)]
pub(crate) fn init_probs(probs: &mut [u16]) {
    probs.fill(PROB_INIT);
}
