// Generated macro for DateTime (struct)
macro_rules! Depcrate_wrappersDateTime {
() => {
// Module: crate::wrappers
// Provides: {"DateTime"}
// Dependencies: {}
# [doc = " A wrapper type for [`jiff::civil::DateTime`]."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , PartialOrd , Ord , diesel :: expression :: AsExpression , diesel :: deserialize :: FromSqlRow ,)] # [diesel (sql_type = diesel :: sql_types :: Timestamp)] pub struct DateTime (jiff :: civil :: DateTime) ;
};
}
