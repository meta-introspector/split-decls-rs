use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub(crate) struct LzmaCoder {
    pub(crate) pos_mask: u32,
    pub(crate) reps: [i32; REPS],
    pub(crate) state: State,
    pub(crate) is_match: [[u16; POS_STATES_MAX]; STATES],
    pub(crate) is_rep: [u16; STATES],
    pub(crate) is_rep0: [u16; STATES],
    pub(crate) is_rep1: [u16; STATES],
    pub(crate) is_rep2: [u16; STATES],
    pub(crate) is_rep0_long: [[u16; POS_STATES_MAX]; STATES],
    pub(crate) dist_slots: [[u16; DIST_SLOTS]; DIST_STATES],
    dist_special: [u16; 124],
    dist_align: [u16; ALIGN_SIZE],
}
