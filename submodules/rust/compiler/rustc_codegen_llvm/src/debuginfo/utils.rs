mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_middle :: ty :: layout :: { HasTypingEnv , LayoutOf } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty } ;}
mkuse!{use tracing :: trace ;}
mkuse!{use super :: CodegenUnitDebugContext ;}
mkuse!{use super :: namespace :: item_namespace ;}
mkuse!{use crate :: common :: CodegenCx ;}
mkuse!{use crate :: llvm ;}
mkuse!{use crate :: llvm :: debuginfo :: { DIArray , DIBuilder , DIDescriptor , DIScope } ;}

macro_rules! is_node_local_to_unit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_node_local_to_unit in module {}", module_path!());
    };
}

mkfn!{
    is_node_local_to_unit_introspect!();
    pub (crate) fn is_node_local_to_unit (cx : & CodegenCx < '_ , '_ > , def_id : DefId) -> bool { ! cx . tcx . is_reachable_non_generic (def_id) }
}

macro_rules! create_DIArray_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_DIArray in module {}", module_path!());
    };
}

mkfn!{
    create_DIArray_introspect!();
    # [allow (non_snake_case)] pub (crate) fn create_DIArray < 'll > (builder : & DIBuilder < 'll > , arr : & [Option < & 'll DIDescriptor >] ,) -> & 'll DIArray { unsafe { llvm :: LLVMRustDIBuilderGetOrCreateArray (builder , arr . as_ptr () , arr . len () as u32) } }
}

macro_rules! debug_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function debug_context in module {}", module_path!());
    };
}

mkfn!{
    debug_context_introspect!();
    # [inline] pub (crate) fn debug_context < 'a , 'll , 'tcx > (cx : & 'a CodegenCx < 'll , 'tcx > ,) -> & 'a CodegenUnitDebugContext < 'll , 'tcx > { cx . dbg_cx . as_ref () . unwrap () }
}

macro_rules! DIB_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function DIB in module {}", module_path!());
    };
}

mkfn!{
    DIB_introspect!();
    # [inline] # [allow (non_snake_case)] pub (crate) fn DIB < 'a , 'll > (cx : & 'a CodegenCx < 'll , '_ >) -> & 'a DIBuilder < 'll > { cx . dbg_cx . as_ref () . unwrap () . builder . as_ref () }
}

macro_rules! get_namespace_for_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_namespace_for_item in module {}", module_path!());
    };
}

mkfn!{
    get_namespace_for_item_introspect!();
    pub (crate) fn get_namespace_for_item < 'll > (cx : & CodegenCx < 'll , '_ > , def_id : DefId) -> & 'll DIScope { item_namespace (cx , cx . tcx . parent (def_id)) }
}
mkitem!{mkenum!{# [derive (Debug , PartialEq , Eq)] pub (crate) enum WidePtrKind { Slice , Dyn , }}}

macro_rules! wide_pointer_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wide_pointer_kind in module {}", module_path!());
    };
}

mkfn!{
    wide_pointer_kind_introspect!();
    # [doc = " Determines if `pointee_ty` is slice-like or trait-object-like, i.e."] # [doc = " if the second field of the wide pointer is a length or a vtable-pointer."] # [doc = " If `pointee_ty` does not require a wide pointer (because it is Sized) then"] # [doc = " the function returns `None`."] pub (crate) fn wide_pointer_kind < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , pointee_ty : Ty < 'tcx > ,) -> Option < WidePtrKind > { let pointee_tail_ty = cx . tcx . struct_tail_for_codegen (pointee_ty , cx . typing_env ()) ; let layout = cx . layout_of (pointee_tail_ty) ; trace ! ("wide_pointer_kind: {:?} has layout {:?} (is_unsized? {})" , pointee_tail_ty , layout , layout . is_unsized ()) ; if layout . is_sized () { return None ; } match * pointee_tail_ty . kind () { ty :: Str | ty :: Slice (_) => Some (WidePtrKind :: Slice) , ty :: Dynamic (..) => Some (WidePtrKind :: Dyn) , ty :: Foreign (_) => { assert_eq ! (cx . size_of (Ty :: new_imm_ptr (cx . tcx , pointee_tail_ty)) , cx . size_of (Ty :: new_imm_ptr (cx . tcx , cx . tcx . types . u8))) ; None } _ => { panic ! ("wide_pointer_kind() - Encountered unexpected `pointee_tail_ty`: {pointee_tail_ty:?}") } } }
}