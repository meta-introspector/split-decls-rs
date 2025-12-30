// Generated macro for Arena (struct)
macro_rules! Depcrate_bridge_arenaArena {
() => {
// Module: crate::bridge::arena
// Provides: {"Arena"}
// Dependencies: {}
# [doc = " A minimal arena allocator inspired by `rustc_arena::DroplessArena`."] # [doc = ""] # [doc = " This is unfortunately a complete re-implementation rather than a dependency"] # [doc = " as it is difficult to depend on crates from within `proc_macro`, due to it"] # [doc = " being built at the same time as `std`."] # [doc = ""] # [doc = " This arena doesn't have support for allocating anything other than byte"] # [doc = " slices, as that is all that is necessary."] pub (crate) struct Arena { start : Cell < * mut MaybeUninit < u8 > > , end : Cell < * mut MaybeUninit < u8 > > , chunks : RefCell < Vec < Box < [MaybeUninit < u8 >] > > > , }
};
}
