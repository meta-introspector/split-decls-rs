// Generated macro for foreign_derives (module)
macro_rules! Depcrate_pg_types_ipnet_addressforeign_derives {
() => {
// Module: crate::pg::types::ipnet_address
// Provides: {"foreign_derives"}
// Dependencies: {}
# [allow (dead_code)] mod foreign_derives { use super :: * ; use crate :: expression :: AsExpression ; # [derive (AsExpression , FromSqlRow)] # [diesel (foreign_derive)] # [diesel (sql_type = Inet)] # [diesel (sql_type = Cidr)] struct IpNetworkProxy (IpNet) ; }
};
}
