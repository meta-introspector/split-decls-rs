// Generated macro for foreign_derives (module)
macro_rules! Depcrate_pg_types_mac_addrforeign_derives {
() => {
// Module: crate::pg::types::mac_addr
// Provides: {"foreign_derives"}
// Dependencies: {}
# [allow (dead_code)] mod foreign_derives { use super :: * ; use crate :: deserialize :: FromSqlRow ; use crate :: expression :: AsExpression ; # [derive (AsExpression , FromSqlRow)] # [diesel (foreign_derive)] # [diesel (sql_type = MacAddr)] struct ByteArrayProxy ([u8 ; 6]) ; }
};
}
