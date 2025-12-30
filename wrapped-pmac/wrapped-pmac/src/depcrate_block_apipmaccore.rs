// Generated macro for PmacCore (struct)
macro_rules! Depcrate_block_apiPmacCore {
() => {
// Module: crate::block_api
// Provides: {"PmacCore"}
// Dependencies: {}
# [doc = " Generic PMAC instance"] # [doc = ""] # [doc = " `LC_SIZE` regulates size of pre-computed table used in PMAC computation."] # [doc = " With `LC_SIZE = 20` and for 128-bit block cipher the table is sufficient"] # [doc = " for 16*2^20 = 16 MiB of input data. For longer messages the `l` value will"] # [doc = " be computed on the fly from the last table value, which will be a bit less"] # [doc = " efficient."] # [derive (Clone)] pub struct PmacCore < C : PmacCipher , const LC_SIZE : usize = 20 > { state : PmacState < C , LC_SIZE > , cipher : C , }
};
}
