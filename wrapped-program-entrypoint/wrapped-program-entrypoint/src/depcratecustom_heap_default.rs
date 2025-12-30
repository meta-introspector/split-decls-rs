// Generated macro for custom_heap_default (macro)
macro_rules! Depcratecustom_heap_default {
() => {
// Module: crate
// Provides: {"custom_heap_default"}
// Dependencies: {}
# [doc = " Define the default global allocator."] # [doc = ""] # [doc = " The default global allocator is enabled only if the calling crate has not"] # [doc = " disabled it using [Cargo features] as described below. It is only defined"] # [doc = " for [BPF] targets."] # [doc = ""] # [doc = " [Cargo features]: https://doc.rust-lang.org/cargo/reference/features.html"] # [doc = " [BPF]: https://solana.com/docs/programs/faq#berkeley-packet-filter-bpf"] # [doc = ""] # [doc = " # Cargo features"] # [doc = ""] # [doc = " A crate that calls this macro can provide its own custom heap"] # [doc = " implementation, or allow others to provide their own custom heap"] # [doc = " implementation, by adding a `custom-heap` feature to its `Cargo.toml`. After"] # [doc = " enabling the feature, one may define their own [global allocator] in the"] # [doc = " standard way."] # [doc = ""] # [doc = " [global allocator]: https://doc.rust-lang.org/stable/std/alloc/trait.GlobalAlloc.html"] # [doc = ""] # [macro_export] macro_rules ! custom_heap_default { () => { # [cfg (all (not (feature = "custom-heap") , target_os = "solana"))] # [global_allocator] static A : $ crate :: BumpAllocator = unsafe { $ crate :: BumpAllocator :: with_fixed_address_range ($ crate :: HEAP_START_ADDRESS as usize , $ crate :: HEAP_LENGTH ,) } ; } ; }
};
}
