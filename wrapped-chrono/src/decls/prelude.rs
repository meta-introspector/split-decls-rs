macro_rules! prelude {
    () => {
        # [doc = " A convenience module appropriate for glob imports (`use chrono::prelude::*;`)."] pub mod prelude { # [allow (deprecated)] pub use crate :: Date ; # [cfg (feature = "clock")] pub use crate :: Local ; # [cfg (all (feature = "unstable-locales" , feature = "alloc"))] pub use crate :: Locale ; pub use crate :: SubsecRound ; pub use crate :: { DateTime , SecondsFormat } ; pub use crate :: { Datelike , Month , Timelike , Weekday } ; pub use crate :: { FixedOffset , Utc } ; pub use crate :: { NaiveDate , NaiveDateTime , NaiveTime } ; pub use crate :: { Offset , TimeZone } ; }
    };
}

prelude!()