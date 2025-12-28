macro_rules! KNONVOLATILE_CONTEXT_POINTERS {
    () => {
        # [repr (C)] # [cfg (target_arch = "x86")] # [derive (Clone , Copy)] pub struct KNONVOLATILE_CONTEXT_POINTERS { pub Dummy : u32 , }
    };
}

KNONVOLATILE_CONTEXT_POINTERS!()