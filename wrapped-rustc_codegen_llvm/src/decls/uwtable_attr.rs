macro_rules! deps {
    () => {
        CreateUWTableAttr!();
    };
}

macro_rules! uwtable_attr {
    () => {
        deps!();
        # [doc = " Tell LLVM to emit or not emit the information necessary to unwind the stack for the function."] # [inline] pub (crate) fn uwtable_attr (llcx : & llvm :: Context , use_sync_unwind : Option < bool >) -> & Attribute { let async_unwind = ! use_sync_unwind . unwrap_or (false) ; llvm :: CreateUWTableAttr (llcx , async_unwind) }
    };
}

uwtable_attr!()