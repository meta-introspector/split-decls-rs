macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! catch_unwind_filter {
    () => {
        deps!();
        # [doc = " don't print panic info for Done/Cancel"] fn catch_unwind_filter < F : FnOnce () -> R + panic :: UnwindSafe , R > (f : F) -> std :: thread :: Result < R > { use std :: sync :: Once ; static INIT : Once = Once :: new () ; INIT . call_once (| | { let prev_hook = panic :: take_hook () ; panic :: set_hook (Box :: new (move | info | { if let Some (Error :: Cancel | Error :: Done) = info . payload () . downcast_ref :: < Error > () { return ; } prev_hook (info) ; })) ; }) ; panic :: catch_unwind (f) }
    };
}

catch_unwind_filter!();