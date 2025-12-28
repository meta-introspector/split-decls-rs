macro_rules! deps {
    () => {
        Visibility!();
        CodegenCx!();
        CallConv!();
        UnnamedAddr!();
        SmallVec!();
        AttributeKind!();
    };
}

macro_rules! declare_raw_fn {
    () => {
        deps!();
        # [doc = " Declare a function."] # [doc = ""] # [doc = " If there’s a value with the same name already declared, the function will"] # [doc = " update the declaration and return existing Value instead."] pub (crate) fn declare_raw_fn < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , name : & str , callconv : llvm :: CallConv , unnamed : llvm :: UnnamedAddr , visibility : llvm :: Visibility , ty : & 'll Type ,) -> & 'll Value { debug ! ("declare_raw_fn(name={:?}, ty={:?})" , name , ty) ; let llfn = declare_simple_fn (cx , name , callconv , unnamed , visibility , ty) ; let mut attrs = SmallVec :: < [_ ; 4] > :: new () ; if cx . tcx . sess . opts . cg . no_redzone . unwrap_or (cx . tcx . sess . target . disable_redzone) { attrs . push (llvm :: AttributeKind :: NoRedZone . create_attr (cx . llcx)) ; } attrs . extend (attributes :: non_lazy_bind_attr (cx)) ; attributes :: apply_to_llfn (llfn , Function , & attrs) ; llfn }
    };
}

declare_raw_fn!();