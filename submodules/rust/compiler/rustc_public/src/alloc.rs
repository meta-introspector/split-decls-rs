mkuse!{use rustc_abi :: Align ;}
mkuse!{use rustc_middle :: mir :: ConstValue ;}
mkuse!{use rustc_middle :: mir :: interpret :: AllocRange ;}
mkuse!{use rustc_public_bridge :: bridge :: Error as _ ;}
mkuse!{use rustc_public_bridge :: context :: CompilerCtxt ;}
mkuse!{use rustc_public_bridge :: { Tables , alloc } ;}
mkuse!{use super :: Error ;}
mkuse!{use super :: compiler_interface :: BridgeTys ;}
mkuse!{use super :: mir :: Mutability ;}
mkuse!{use super :: ty :: { Allocation , ProvenanceMap } ;}
mkuse!{use super :: unstable :: Stable ;}

macro_rules! new_empty_allocation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_empty_allocation in module {}", module_path!());
    };
}

mkfn!{
    new_empty_allocation_introspect!();
    # [doc = " Creates new empty `Allocation` from given `Align`."] fn new_empty_allocation (align : Align) -> Allocation { Allocation { bytes : Vec :: new () , provenance : ProvenanceMap { ptrs : Vec :: new () } , align : align . bytes () , mutability : Mutability :: Not , } }
}

macro_rules! new_allocation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_allocation in module {}", module_path!());
    };
}

mkfn!{
    new_allocation_introspect!();
    # [allow (rustc :: usage_of_qualified_ty)] pub (crate) fn new_allocation < 'tcx > (ty : rustc_middle :: ty :: Ty < 'tcx > , const_value : ConstValue , tables : & mut Tables < 'tcx , BridgeTys > , cx : & CompilerCtxt < 'tcx , BridgeTys > ,) -> Allocation { try_new_allocation (ty , const_value , tables , cx) . unwrap_or_else (| _ | panic ! ("Failed to convert: {const_value:?} to {ty:?}")) }
}

macro_rules! try_new_allocation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_new_allocation in module {}", module_path!());
    };
}

mkfn!{
    try_new_allocation_introspect!();
    # [allow (rustc :: usage_of_qualified_ty)] pub (crate) fn try_new_allocation < 'tcx > (ty : rustc_middle :: ty :: Ty < 'tcx > , const_value : ConstValue , tables : & mut Tables < 'tcx , BridgeTys > , cx : & CompilerCtxt < 'tcx , BridgeTys > ,) -> Result < Allocation , Error > { let layout = alloc :: create_ty_and_layout (cx , ty) . map_err (| e | Error :: from_internal (e)) ? ; match const_value { ConstValue :: Scalar (scalar) => { alloc :: try_new_scalar (layout , scalar , cx) . map (| alloc | alloc . stable (tables , cx)) } ConstValue :: ZeroSized => Ok (new_empty_allocation (layout . align . abi)) , ConstValue :: Slice { alloc_id , meta } => { alloc :: try_new_slice (layout , alloc_id , meta , cx) . map (| alloc | alloc . stable (tables , cx)) } ConstValue :: Indirect { alloc_id , offset } => { let alloc = alloc :: try_new_indirect (alloc_id , cx) ; use rustc_public_bridge :: context :: AllocRangeHelpers ; Ok (allocation_filter (& alloc . 0 , cx . alloc_range (offset , layout . size) , tables , cx)) } } }
}

macro_rules! allocation_filter_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function allocation_filter in module {}", module_path!());
    };
}

mkfn!{
    allocation_filter_introspect!();
    # [doc = " Creates an `Allocation` only from information within the `AllocRange`."] pub (super) fn allocation_filter < 'tcx > (alloc : & rustc_middle :: mir :: interpret :: Allocation , alloc_range : AllocRange , tables : & mut Tables < 'tcx , BridgeTys > , cx : & CompilerCtxt < 'tcx , BridgeTys > ,) -> Allocation { alloc :: allocation_filter (alloc , alloc_range , tables , cx) }
}