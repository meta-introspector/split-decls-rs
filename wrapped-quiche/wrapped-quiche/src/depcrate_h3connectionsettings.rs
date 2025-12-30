// Generated macro for ConnectionSettings (struct)
macro_rules! Depcrate_h3ConnectionSettings {
() => {
// Module: crate::h3
// Provides: {"ConnectionSettings"}
// Dependencies: {}
struct ConnectionSettings { pub max_field_section_size : Option < u64 > , pub qpack_max_table_capacity : Option < u64 > , pub qpack_blocked_streams : Option < u64 > , pub connect_protocol_enabled : Option < u64 > , pub h3_datagram : Option < u64 > , pub additional_settings : Option < Vec < (u64 , u64) > > , pub raw : Option < Vec < (u64 , u64) > > , }
};
}
