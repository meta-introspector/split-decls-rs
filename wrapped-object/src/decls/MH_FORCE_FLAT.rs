macro_rules! MH_FORCE_FLAT {
    () => {
        # [doc = " the executable is forcing all images to use flat name space bindings"] pub const MH_FORCE_FLAT : u32 = 0x100 ;
    };
}

MH_FORCE_FLAT!()