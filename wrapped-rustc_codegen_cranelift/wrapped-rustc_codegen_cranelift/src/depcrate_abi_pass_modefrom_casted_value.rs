// Generated macro for from_casted_value (function)
macro_rules! Depcrate_abi_pass_modefrom_casted_value {
() => {
// Module: crate::abi::pass_mode
// Provides: {"from_casted_value"}
// Dependencies: {}
pub (super) fn from_casted_value < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , block_params : & [Value] , layout : TyAndLayout < 'tcx > , cast : & CastTarget ,) -> CValue < 'tcx > { let abi_params = cast_target_to_abi_params (cast) ; let abi_param_size : u32 = abi_params . iter () . map (| (_ , param) | param . value_type . bytes ()) . sum () ; let layout_size = u32 :: try_from (layout . size . bytes ()) . unwrap () ; let ptr = fx . create_stack_slot (std :: cmp :: max (abi_param_size , layout_size) , u32 :: try_from (layout . align . abi . bytes ()) . unwrap () ,) ; let mut block_params_iter = block_params . iter () . copied () ; for (offset , _) in abi_params { ptr . offset_i64 (fx , offset . bytes () as i64) . store (fx , block_params_iter . next () . unwrap () , MemFlags :: new () ,) } assert_eq ! (block_params_iter . next () , None , "Leftover block param") ; CValue :: by_ref (ptr , layout) }
};
}
