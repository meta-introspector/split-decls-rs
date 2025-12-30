// Generated macro for Output (type)
macro_rules! Depcrate_ots_signatureOutput {
() => {
// Module: crate::ots::signature
// Provides: {"Output"}
// Dependencies: {}
# [doc = " Useful type alias to get the [`Array`] representation"] pub type Output < Mode > = Array < u8 , Sum < Prod < < Mode as LmsOtsMode > :: NLen , Sum < < Mode as LmsOtsMode > :: PLen , U1 > > , U4 > > ;
};
}
