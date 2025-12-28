macro_rules! deps {
    () => {
        Visibility!();
        SimpleCx!();
        UnnamedAddr!();
        SetFunctionCallConv!();
        CallConv!();
    };
}

macro_rules! declare_simple_fn {
    () => {
        deps!();
        # [doc = " Declare a function with a SimpleCx."] # [doc = ""] # [doc = " If there’s a value with the same name already declared, the function will"] # [doc = " update the declaration and return existing Value instead."] pub (crate) fn declare_simple_fn < 'll > (cx : & SimpleCx < 'll > , name : & str , callconv : llvm :: CallConv , unnamed : llvm :: UnnamedAddr , visibility : llvm :: Visibility , ty : & 'll Type ,) -> & 'll Value { debug ! ("declare_simple_fn(name={:?}, ty={:?})" , name , ty) ; let llfn = unsafe { llvm :: LLVMRustGetOrInsertFunction (cx . llmod , name . as_c_char_ptr () , name . len () , ty) } ; llvm :: SetFunctionCallConv (llfn , callconv) ; llvm :: set_unnamed_address (llfn , unnamed) ; llvm :: set_visibility (llfn , visibility) ; llfn }
    };
}

declare_simple_fn!()