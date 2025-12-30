// Generated macro for get_argc_argv (function)
macro_rules! Depcrate_baseget_argc_argv {
() => {
// Module: crate::base
// Provides: {"get_argc_argv"}
// Dependencies: {}
# [doc = " Obtain the `argc` and `argv` values to pass to the rust start function"] # [doc = " (i.e., the \"start\" lang item)."] fn get_argc_argv < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx) -> (Bx :: Value , Bx :: Value) { if bx . cx () . sess () . target . os . contains ("uefi") { let param_handle = bx . get_param (0) ; let param_system_table = bx . get_param (1) ; let ptr_size = bx . tcx () . data_layout . pointer_size () ; let ptr_align = bx . tcx () . data_layout . pointer_align () . abi ; let arg_argc = bx . const_int (bx . cx () . type_isize () , 2) ; let arg_argv = bx . alloca (2 * ptr_size , ptr_align) ; bx . store (param_handle , arg_argv , ptr_align) ; let arg_argv_el1 = bx . inbounds_ptradd (arg_argv , bx . const_usize (ptr_size . bytes ())) ; bx . store (param_system_table , arg_argv_el1 , ptr_align) ; (arg_argc , arg_argv) } else if bx . cx () . sess () . target . main_needs_argc_argv { let param_argc = bx . get_param (0) ; let param_argv = bx . get_param (1) ; let arg_argc = bx . intcast (param_argc , bx . cx () . type_isize () , true) ; let arg_argv = param_argv ; (arg_argc , arg_argv) } else { let arg_argc = bx . const_int (bx . cx () . type_int () , 0) ; let arg_argv = bx . const_null (bx . cx () . type_ptr ()) ; (arg_argc , arg_argv) } }
};
}
