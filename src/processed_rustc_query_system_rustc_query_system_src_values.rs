/* FP:values.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_values_USE_0001
/* FP:values.rs-0002 */ use crate :: rustc_complete :: ErrorGuaranteed ;
/* FP:values.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_values_USE_0002
/* FP:values.rs-0004 */ use crate :: dep_graph :: DepContext ;
/* FP:values.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_values_USE_0003
/* FP:values.rs-0006 */ use crate :: query :: CycleError ;
/* FP:values.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_values_TRAIT_0004
/* FP:values.rs-0008 */ pub trait Value < Tcx : DepContext > : Sized { fn from_cycle_error (tcx : Tcx , cycle_error : & CycleError , guar : ErrorGuaranteed) -> Self ; }
/* FP:values.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_values_IMPL_0005
/* FP:values.rs-0010 */ impl < Tcx : DepContext , T > Value < Tcx > for T { default fn from_cycle_error (tcx : Tcx , cycle_error : & CycleError , _guar : ErrorGuaranteed) -> T { tcx . sess () . dcx () . abort_if_errors () ; panic ! ("<{} as Value>::from_cycle_error called without errors: {:#?}" , std :: any :: type_name ::< T > () , cycle_error . cycle ,) ; } }