// Generated macro for DBCommon (struct)
macro_rules! Depcrate_dbDBCommon {
() => {
// Module: crate::db
// Provides: {"DBCommon"}
// Dependencies: {}
# [doc = " A helper type to implement some common methods for [`DBWithThreadMode`]"] # [doc = " and [`OptimisticTransactionDB`]."] # [doc = ""] # [doc = " [`OptimisticTransactionDB`]: crate::OptimisticTransactionDB"] pub struct DBCommon < T : ThreadMode , D : DBInner > { pub (crate) inner : D , cfs : T , path : PathBuf , _outlive : Vec < OptionsMustOutliveDB > , }
};
}
