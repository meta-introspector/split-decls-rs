// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
mod prelude { pub (crate) use cranelift_codegen :: Context ; pub (crate) use cranelift_codegen :: ir :: condcodes :: { FloatCC , IntCC } ; pub (crate) use cranelift_codegen :: ir :: function :: Function ; pub (crate) use cranelift_codegen :: ir :: { AbiParam , Block , FuncRef , Inst , InstBuilder , MemFlags , Signature , SourceLoc , StackSlot , StackSlotData , StackSlotKind , TrapCode , Type , Value , types , } ; pub (crate) use cranelift_module :: { self , DataDescription , FuncId , Linkage , Module } ; pub (crate) use rustc_abi :: { BackendRepr , FIRST_VARIANT , FieldIdx , Scalar , Size , VariantIdx } ; pub (crate) use rustc_data_structures :: fx :: { FxHashMap , FxIndexMap } ; pub (crate) use rustc_hir :: def_id :: { DefId , LOCAL_CRATE } ; pub (crate) use rustc_index :: Idx ; pub (crate) use rustc_middle :: mir :: { self , * } ; pub (crate) use rustc_middle :: ty :: layout :: { LayoutOf , TyAndLayout } ; pub (crate) use rustc_middle :: ty :: { self , FloatTy , Instance , InstanceKind , IntTy , Ty , TyCtxt , UintTy , } ; pub (crate) use rustc_span :: Span ; pub (crate) use crate :: abi :: * ; pub (crate) use crate :: base :: { codegen_operand , codegen_place } ; pub (crate) use crate :: cast :: * ; pub (crate) use crate :: common :: * ; pub (crate) use crate :: debuginfo :: { DebugContext , UnwindContext } ; pub (crate) use crate :: pointer :: Pointer ; pub (crate) use crate :: value_and_place :: { CPlace , CValue } ; }
};
}
