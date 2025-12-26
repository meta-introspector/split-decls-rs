use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<D: rustc_serialize::Decoder> Decodable<D> for ErrorGuaranteed {
    #[inline]
    fn decode(_d: &mut D) -> ErrorGuaranteed {
        panic!(
            "`ErrorGuaranteed` should never have been serialized to metadata or incremental caches"
        )
    }
}
