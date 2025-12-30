// Generated macro for codegen_bitcast (function)
macro_rules! Depcrate_commoncodegen_bitcast {
() => {
// Module: crate::common
// Provides: {"codegen_bitcast"}
// Dependencies: {}
pub (crate) fn codegen_bitcast (fx : & mut FunctionCx < '_ , '_ , '_ > , dst_ty : Type , val : Value) -> Value { let mut flags = MemFlags :: new () ; flags . set_endianness (match fx . tcx . data_layout . endian { rustc_abi :: Endian :: Big => cranelift_codegen :: ir :: Endianness :: Big , rustc_abi :: Endian :: Little => cranelift_codegen :: ir :: Endianness :: Little , }) ; fx . bcx . ins () . bitcast (dst_ty , flags , val) }
};
}
