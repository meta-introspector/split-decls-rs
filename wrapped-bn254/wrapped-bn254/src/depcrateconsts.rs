// Generated macro for consts (module)
macro_rules! Depcrateconsts {
() => {
// Module: crate
// Provides: {"consts"}
// Dependencies: {}
mod consts { # [doc = " Size of the EC point field, in bytes."] pub const ALT_BN128_FIELD_SIZE : usize = 32 ; # [doc = " Size of the EC point. `alt_bn128` point contains"] # [doc = " the consistently united x and y fields as 64 bytes."] pub const ALT_BN128_G1_POINT_SIZE : usize = ALT_BN128_FIELD_SIZE * 2 ; # [deprecated (since = "3.1.0" , note = "Please use `ALT_BN128_G1_POINT_SIZE` instead")] pub const ALT_BN128_POINT_SIZE : usize = ALT_BN128_G1_POINT_SIZE ; # [doc = " Elements in G2 is represented by 2 field-extension elements `(x, y)`."] pub const ALT_BN128_G2_POINT_SIZE : usize = ALT_BN128_FIELD_SIZE * 4 ; }
};
}
