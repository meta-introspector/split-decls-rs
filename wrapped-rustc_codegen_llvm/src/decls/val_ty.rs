macro_rules! val_ty {
    () => {
        # [doc = " Get the [LLVM type][Type] of a [`Value`]."] pub (crate) fn val_ty (v : & Value) -> & Type { unsafe { llvm :: LLVMTypeOf (v) } }
    };
}

val_ty!();