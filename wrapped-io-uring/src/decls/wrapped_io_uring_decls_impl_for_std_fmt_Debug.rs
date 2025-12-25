use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl std::fmt::Debug for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parameters")
            .field("is_setup_sqpoll", &self.is_setup_sqpoll())
            .field("is_setup_iopoll", &self.is_setup_iopoll())
            .field("is_setup_single_issuer", &self.is_setup_single_issuer())
            .field("is_feature_single_mmap", &self.is_feature_single_mmap())
            .field("is_feature_nodrop", &self.is_feature_nodrop())
            .field("is_feature_submit_stable", &self.is_feature_submit_stable())
            .field("is_feature_rw_cur_pos", &self.is_feature_rw_cur_pos())
            .field(
                "is_feature_cur_personality",
                &self.is_feature_cur_personality(),
            )
            .field("is_feature_poll_32bits", &self.is_feature_poll_32bits())
            .field("sq_entries", &self.0.sq_entries)
            .field("cq_entries", &self.0.cq_entries)
            .finish()
    }
}
