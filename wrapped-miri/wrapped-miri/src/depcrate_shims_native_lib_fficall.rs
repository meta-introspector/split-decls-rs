// Generated macro for call (function)
macro_rules! Depcrate_shims_native_lib_fficall {
() => {
// Module: crate::shims::native_lib::ffi
// Provides: {"call"}
// Dependencies: {}
# [doc = " Perform the actual FFI call."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The safety invariants of the foreign function being called must be upheld (if any)."] pub unsafe fn call < R : libffi :: high :: CType > (fun : CodePtr , args : & mut [OwnedArg]) -> R { let arg_ptrs : Vec < _ > = args . iter () . map (| arg | arg . ptr ()) . collect () ; let cif = Cif :: new (args . iter_mut () . map (| arg | arg . ty . take () . unwrap ()) , R :: reify () . into_middle ()) ; unsafe { cif . call (fun , & arg_ptrs) } }
};
}
