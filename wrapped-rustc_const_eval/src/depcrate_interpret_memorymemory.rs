// Generated macro for Memory (struct)
macro_rules! Depcrate_interpret_memoryMemory {
() => {
// Module: crate::interpret::memory
// Provides: {"Memory"}
// Dependencies: {}
pub struct Memory < 'tcx , M : Machine < 'tcx > > { # [doc = " Allocations local to this instance of the interpreter. The kind"] # [doc = " helps ensure that the same mechanism is used for allocation and"] # [doc = " deallocation. When an allocation is not found here, it is a"] # [doc = " global and looked up in the `tcx` for read access. Some machines may"] # [doc = " have to mutate this map even on a read-only access to a global (because"] # [doc = " they do pointer provenance tracking and the allocations in `tcx` have"] # [doc = " the wrong type), so we let the machine override this type."] # [doc = " Either way, if the machine allows writing to a global, doing so will"] # [doc = " create a copy of the global allocation here."] pub (super) alloc_map : M :: MemoryMap , # [doc = " Map for \"extra\" function pointers."] extra_fn_ptr_map : FxIndexMap < AllocId , M :: ExtraFnVal > , # [doc = " To be able to compare pointers with null, and to check alignment for accesses"] # [doc = " to ZSTs (where pointers may dangle), we keep track of the size even for allocations"] # [doc = " that do not exist any more."] pub (super) dead_alloc_map : FxIndexMap < AllocId , (Size , Align) > , # [doc = " This stores whether we are currently doing reads purely for the purpose of validation."] # [doc = " Those reads do not trigger the machine's hooks for memory reads."] # [doc = " Needless to say, this must only be set with great care!"] validation_in_progress : Cell < bool > , }
};
}
