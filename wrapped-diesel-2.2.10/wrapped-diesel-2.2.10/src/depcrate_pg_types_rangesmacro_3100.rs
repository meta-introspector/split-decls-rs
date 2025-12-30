// Generated macro for macro_3100 (macro)
macro_rules! Depcrate_pg_types_rangesmacro_3100 {
() => {
// Module: crate::pg::types::ranges
// Provides: {"macro_3100"}
// Dependencies: {}
bitflags :: bitflags ! { struct RangeFlags : u8 { const EMPTY = 0x01 ; const LB_INC = 0x02 ; const UB_INC = 0x04 ; const LB_INF = 0x08 ; const UB_INF = 0x10 ; const LB_NULL = 0x20 ; const UB_NULL = 0x40 ; const CONTAIN_EMPTY = 0x80 ; } }
};
}
