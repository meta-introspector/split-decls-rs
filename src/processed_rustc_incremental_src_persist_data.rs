/* FP:data.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_data_USE_0001
/* FP:data.rs-0002 */ use rustc_macros :: { Decodable , Encodable } ;
/* FP:data.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_data_USE_0002
/* FP:data.rs-0004 */ use crate :: rustc_complete :: dep_graph :: { WorkProduct , WorkProductId } ;
/* FP:data.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_data_STRUCT_0003
/* FP:data.rs-0006 */ # [derive (Debug , Encodable , Decodable)] pub (crate) struct SerializedWorkProduct { # [doc = " node that produced the work-product"] pub id : WorkProductId , # [doc = " work-product data itself"] pub work_product : WorkProduct , }