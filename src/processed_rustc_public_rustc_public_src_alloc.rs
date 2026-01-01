/* FP:alloc.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_USE_0001
/* FP:alloc.rs-0002 */ use crate :: rustc_abi :: Align ;
/* FP:alloc.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_USE_0002
/* FP:alloc.rs-0004 */ use crate :: rustc_complete :: mir :: ConstValue ;
/* FP:alloc.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_USE_0003
/* FP:alloc.rs-0006 */ use crate :: rustc_complete :: mir :: interpret :: AllocRange ;
/* FP:alloc.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_USE_0004
/* FP:alloc.rs-0008 */ use crate :: rustc_public_bridge :: bridge :: Error as _ ;
/* FP:alloc.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_USE_0005
/* FP:alloc.rs-0010 */ use crate :: rustc_public_bridge :: context :: CompilerCtxt ;
/* FP:alloc.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_USE_0006
/* FP:alloc.rs-0012 */ use crate :: rustc_public_bridge :: { Tables , alloc } ;
/* FP:alloc.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_USE_0007
/* FP:alloc.rs-0014 */ use super :: Error ;
/* FP:alloc.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_USE_0008
/* FP:alloc.rs-0016 */ use super :: compiler_interface :: BridgeTys ;
/* FP:alloc.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_USE_0009
/* FP:alloc.rs-0018 */ use super :: mir :: Mutability ;
/* FP:alloc.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_USE_0010
/* FP:alloc.rs-0020 */ use super :: ty :: { Allocation , ProvenanceMap } ;
/* FP:alloc.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_USE_0011
/* FP:alloc.rs-0022 */ use super :: unstable :: Stable ;
/* FP:alloc.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_FN_0012
/* FP:alloc.rs-0024 */ # [doc = " Creates new empty `Allocation` from given `Align`."] fn new_empty_allocation (align : Align) -> Allocation { Allocation { bytes : Vec :: new () , provenance : ProvenanceMap { ptrs : Vec :: new () } , align : align . bytes () , mutability : Mutability :: Not , } }
/* FP:alloc.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_FN_0013
/* FP:alloc.rs-0026 */ # [allow (rustc :: usage_of_qualified_ty)] pub (crate) fn new_allocation < 'tcx > (ty : crate :: rustc_middle :: ty :: Ty < 'tcx > , const_value : ConstValue , tables : & mut Tables < 'tcx , BridgeTys > , cx : & CompilerCtxt < 'tcx , BridgeTys > ,) -> Allocation { try_new_allocation (ty , const_value , tables , cx) . unwrap_or_else (| _ | panic ! ("Failed to convert: {const_value:?} to {ty:?}")) }
/* FP:alloc.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_FN_0014
/* FP:alloc.rs-0028 */ # [allow (rustc :: usage_of_qualified_ty)] pub (crate) fn try_new_allocation < 'tcx > (ty : crate :: rustc_middle :: ty :: Ty < 'tcx > , const_value : ConstValue , tables : & mut Tables < 'tcx , BridgeTys > , cx : & CompilerCtxt < 'tcx , BridgeTys > ,) -> Result < Allocation , Error > { let layout = alloc :: create_ty_and_layout (cx , ty) . map_err (| e | Error :: from_internal (e)) ? ; match const_value { ConstValue :: Scalar (scalar) => { alloc :: try_new_scalar (layout , scalar , cx) . map (| alloc | alloc . stable (tables , cx)) } ConstValue :: ZeroSized => Ok (new_empty_allocation (layout . align . abi)) , ConstValue :: Slice { alloc_id , meta } => { alloc :: try_new_slice (layout , alloc_id , meta , cx) . map (| alloc | alloc . stable (tables , cx)) } ConstValue :: Indirect { alloc_id , offset } => { let alloc = alloc :: try_new_indirect (alloc_id , cx) ; use crate :: rustc_public_bridge :: context :: AllocRangeHelpers ; Ok (allocation_filter (& alloc . 0 , cx . alloc_range (offset , layout . size) , tables , cx)) } } }
/* FP:alloc.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_alloc_FN_0015
/* FP:alloc.rs-0030 */ # [doc = " Creates an `Allocation` only from information within the `AllocRange`."] pub (super) fn allocation_filter < 'tcx > (alloc : & crate :: rustc_middle :: mir :: interpret :: Allocation , alloc_range : AllocRange , tables : & mut Tables < 'tcx , BridgeTys > , cx : & CompilerCtxt < 'tcx , BridgeTys > ,) -> Allocation { alloc :: allocation_filter (alloc , alloc_range , tables , cx) }