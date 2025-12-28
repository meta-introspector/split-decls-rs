macro_rules! deps {
    () => {
        ConstEvalErrKind!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl MachineStopType for ConstEvalErrKind { fn diagnostic_message (& self) -> DiagMessage { use ConstEvalErrKind :: * ; use crate :: fluent_generated :: * ; match self { ConstAccessesMutGlobal => const_eval_const_accesses_mut_global , ModifiedGlobal => const_eval_modified_global , Panic { .. } => const_eval_panic , RecursiveStatic => const_eval_recursive_static , AssertFailure (x) => x . diagnostic_message () , WriteThroughImmutablePointer => const_eval_write_through_immutable_pointer , ConstMakeGlobalPtrAlreadyMadeGlobal { .. } => { const_eval_const_make_global_ptr_already_made_global } ConstMakeGlobalPtrIsNonHeap (_) => const_eval_const_make_global_ptr_is_non_heap , ConstMakeGlobalWithDanglingPtr (_) => const_eval_const_make_global_with_dangling_ptr , ConstMakeGlobalWithOffset (_) => const_eval_const_make_global_with_offset , } } fn add_args (self : Box < Self > , adder : & mut dyn FnMut (DiagArgName , DiagArgValue)) { use ConstEvalErrKind :: * ; match * self { RecursiveStatic | ConstAccessesMutGlobal | ModifiedGlobal | WriteThroughImmutablePointer => { } AssertFailure (kind) => kind . add_args (adder) , Panic { msg , .. } => { adder ("msg" . into () , msg . into_diag_arg (& mut None)) ; } ConstMakeGlobalPtrIsNonHeap (ptr) | ConstMakeGlobalWithOffset (ptr) | ConstMakeGlobalWithDanglingPtr (ptr) => { adder ("ptr" . into () , format ! ("{ptr:?}") . into_diag_arg (& mut None)) ; } ConstMakeGlobalPtrAlreadyMadeGlobal (alloc) => { adder ("alloc" . into () , alloc . into_diag_arg (& mut None)) ; } } } }
    };
}

impl_88!();