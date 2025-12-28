macro_rules! MH_DEAD_STRIPPABLE_DYLIB {
    () => {
        # [doc = " Only for use on dylibs.  When linking against a dylib that has this bit set, the static linker will automatically not create a LC_LOAD_DYLIB load command to the dylib if no symbols are being referenced from the dylib."] pub const MH_DEAD_STRIPPABLE_DYLIB : u32 = 0x40_0000 ;
    };
}

MH_DEAD_STRIPPABLE_DYLIB!()