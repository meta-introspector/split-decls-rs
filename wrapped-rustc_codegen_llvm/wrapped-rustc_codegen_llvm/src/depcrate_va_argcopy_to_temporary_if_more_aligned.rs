// Generated macro for copy_to_temporary_if_more_aligned (function)
macro_rules! Depcrate_va_argcopy_to_temporary_if_more_aligned {
() => {
// Module: crate::va_arg
// Provides: {"copy_to_temporary_if_more_aligned"}
// Dependencies: {}
# [doc = " Copy into a temporary if the type is more aligned than the register save area."] fn copy_to_temporary_if_more_aligned < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , reg_addr : & 'll Value , layout : TyAndLayout < 'tcx , Ty < 'tcx > > , src_align : Align ,) -> & 'll Value { if layout . layout . align . abi > src_align { let tmp = bx . alloca (layout . layout . size () , layout . layout . align () . abi) ; bx . memcpy (tmp , layout . layout . align . abi , reg_addr , src_align , bx . const_u32 (layout . layout . size () . bytes () as u32) , MemFlags :: empty () ,) ; tmp } else { reg_addr } }
};
}
