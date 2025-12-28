macro_rules! deps {
    () => {
        MinWord!();
        RetInt!();
    };
}

macro_rules! create_sub_word_mask_values {
    () => {
        deps!();
        # [cfg (not (target_pointer_width = "16"))] # [allow (dead_code)] # [inline] pub (crate) fn create_sub_word_mask_values < T > (ptr : * mut T) -> (* mut MinWord , RetInt , RetInt) { # [cfg (atomic_maybe_uninit_no_strict_provenance)] use self :: ptr :: MutPtrExt as _ ; const SHIFT_MASK : bool = ! cfg ! (any (target_arch = "bpf" , target_arch = "loongarch32" , target_arch = "loongarch64" , target_arch = "mips" , target_arch = "mips32r6" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "riscv32" , target_arch = "riscv64" , target_arch = "s390x" , target_arch = "sparc" , target_arch = "sparc64" , target_arch = "xtensa" ,)) ; const PTR_MASK : usize = mem :: size_of :: < MinWord > () - 1 ; const PTR_INV_MASK : usize = ! PTR_MASK ; let aligned_ptr = ptr . with_addr (ptr . addr () & PTR_INV_MASK) . cast :: < MinWord > () ; let ptr_lsb = if SHIFT_MASK { ptr . addr () & PTR_MASK } else { ptr . addr () } ; # [allow (clippy :: arithmetic_side_effects)] let shift = if cfg ! (any (target_endian = "little" , target_arch = "s390x")) { ptr_lsb << 3 } else { (ptr_lsb ^ const_eval ! (T => usize { mem :: size_of ::< MinWord > () - mem :: size_of ::< T > () })) << 3 } ; # [allow (clippy :: arithmetic_side_effects)] let mut mask = const_eval ! (T => RetInt { (1 << (mem :: size_of ::< T > () << 3)) - 1 }) ; if SHIFT_MASK { mask <<= shift ; } # [cfg_attr (any (target_arch = "s390x" , target_pointer_width = "32") , allow (clippy :: cast_possible_truncation))] { (aligned_ptr , shift as RetInt , mask) } }
    };
}

create_sub_word_mask_values!();