// Generated macro for Time (struct)
macro_rules! Depcrate_wrappersTime {
() => {
// Module: crate::wrappers
// Provides: {"Time"}
// Dependencies: {}
# [doc = " A wrapper type for [`jiff::civil::Time`]."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , PartialOrd , Ord , diesel :: expression :: AsExpression , diesel :: deserialize :: FromSqlRow ,)] # [diesel (sql_type = diesel :: sql_types :: Time)] pub struct Time (jiff :: civil :: Time) ;
};
}
