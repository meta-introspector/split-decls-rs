// Generated macro for Date (struct)
macro_rules! Depcrate_wrappersDate {
() => {
// Module: crate::wrappers
// Provides: {"Date"}
// Dependencies: {}
# [doc = " A wrapper type for [`jiff::civil::Date`]."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , PartialOrd , Ord , diesel :: expression :: AsExpression , diesel :: deserialize :: FromSqlRow ,)] # [diesel (sql_type = diesel :: sql_types :: Date)] pub struct Date (jiff :: civil :: Date) ;
};
}
