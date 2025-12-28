macro_rules! deps {
    () => {
        Memory!();
        MemPlace!();
    };
}

macro_rules! Place {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] pub (super) enum Place < Prov : Provenance = CtfeProvenance > { # [doc = " A place referring to a value allocated in the `Memory` system."] Ptr (MemPlace < Prov >) , # [doc = " To support alloc-free locals, we are able to write directly to a local. The offset indicates"] # [doc = " where in the local this place is located; if it is `None`, no projection has been applied"] # [doc = " and the type of the place is exactly the type of the local."] # [doc = " Such projections are meaningful even if the offset is 0, since they can change layouts."] # [doc = " (Without that optimization, we'd just always be a `MemPlace`.)"] # [doc = " `Local` places always refer to the current stack frame, so they are unstable under"] # [doc = " function calls/returns and switching betweens stacks of different threads!"] # [doc = " We carry around the address of the `locals` buffer of the correct stack frame as a sanity"] # [doc = " check to be able to catch some cases of using a dangling `Place`."] # [doc = ""] # [doc = " This variant shall not be used for unsized types -- those must always live in memory."] Local { local : mir :: Local , offset : Option < Size > , locals_addr : usize } , }
    };
}

Place!();