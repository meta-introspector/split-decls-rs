use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Zero-copy serialization/deserialization with rkyv.
///
/// This module re-exports the `Archived*` versions of chrono's types.
#[cfg(
    any(feature = "rkyv", feature = "rkyv-16", feature = "rkyv-32", feature = "rkyv-64")
)]
pub mod rkyv {
    pub use crate::datetime::ArchivedDateTime;
    pub use crate::month::ArchivedMonth;
    pub use crate::naive::date::ArchivedNaiveDate;
    pub use crate::naive::datetime::ArchivedNaiveDateTime;
    pub use crate::naive::isoweek::ArchivedIsoWeek;
    pub use crate::naive::time::ArchivedNaiveTime;
    pub use crate::offset::fixed::ArchivedFixedOffset;
    #[cfg(feature = "clock")]
    pub use crate::offset::local::ArchivedLocal;
    pub use crate::offset::utc::ArchivedUtc;
    pub use crate::time_delta::ArchivedTimeDelta;
    pub use crate::weekday::ArchivedWeekday;
    /// Alias of [`ArchivedTimeDelta`]
    pub type ArchivedDuration = ArchivedTimeDelta;
}
