// Generated macro for _macro_support (module)
macro_rules! Depcrate_macro_support {
() => {
// Module: crate
// Provides: {"_macro_support"}
// Dependencies: {}
# [doc (hidden)] pub mod _macro_support { pub use crate :: content :: Content ; pub use crate :: env :: { get_cargo_workspace , Workspace } ; pub use crate :: runtime :: { assert_snapshot , with_allow_duplicates , AutoName , BinarySnapshotValue , InlineValue , SnapshotValue , } ; pub use core :: { file , line , module_path } ; pub use std :: { any , env , format , option_env , path , vec } ; # [cfg (feature = "serde")] pub use crate :: serialization :: { serialize_value , SerializationFormat , SnapshotLocation } ; # [cfg (feature = "glob")] pub use crate :: glob :: glob_exec ; # [cfg (feature = "redactions")] pub use crate :: { redaction :: Redaction , redaction :: Selector , serialization :: serialize_value_redacted , } ; }
};
}
