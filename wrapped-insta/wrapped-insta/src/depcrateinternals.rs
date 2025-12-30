// Generated macro for internals (module)
macro_rules! Depcrateinternals {
() => {
// Module: crate
// Provides: {"internals"}
// Dependencies: {}
# [doc = " Exposes some library internals."] # [doc = ""] # [doc = " You're unlikely to want to work with these objects but they"] # [doc = " are exposed for documentation primarily."] # [doc = ""] # [doc = " This module does not follow the same stability guarantees as the rest of the crate and is not"] # [doc = " guaranteed to be compatible between minor versions."] pub mod internals { pub use crate :: content :: Content ; # [cfg (feature = "filters")] pub use crate :: filters :: Filters ; pub use crate :: runtime :: AutoName ; pub use crate :: settings :: SettingsBindDropGuard ; pub use crate :: snapshot :: { MetaData , SnapshotContents } ; # [cfg (feature = "redactions")] pub use crate :: { redaction :: { ContentPath , Redaction } , settings :: Redactions , } ; }
};
}
