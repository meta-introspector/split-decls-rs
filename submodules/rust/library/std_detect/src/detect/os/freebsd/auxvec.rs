mkitem!{mkstruct!{# [doc = " Cache HWCAP bitfields of the ELF Auxiliary Vector."] # [doc = ""] # [doc = " If an entry cannot be read all the bits in the bitfield are set to zero."] # [doc = " This should be interpreted as all the features being disabled."] # [derive (Debug , Copy , Clone)] pub (crate) struct AuxVec { pub hwcap : usize , pub hwcap2 : usize , }}}

macro_rules! auxv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function auxv in module {}", module_path!());
    };
}

mkfn!{
    auxv_introspect!();
    # [doc = " ELF Auxiliary Vector"] # [doc = ""] # [doc = " The auxiliary vector is a memory region in a running ELF program's stack"] # [doc = " composed of (key: usize, value: usize) pairs."] # [doc = ""] # [doc = " The keys used in the aux vector are platform dependent. For FreeBSD, they are"] # [doc = " defined in [sys/elf_common.h][elf_common_h]. The hardware capabilities of a given"] # [doc = " CPU can be queried with the  `AT_HWCAP` and `AT_HWCAP2` keys."] # [doc = ""] # [doc = " Note that run-time feature detection is not invoked for features that can"] # [doc = " be detected at compile-time."] # [doc = ""] # [doc = " [elf_common.h]: https://svnweb.freebsd.org/base/release/12.0.0/sys/sys/elf_common.h?revision=341707"] pub (crate) fn auxv () -> Result < AuxVec , () > { let hwcap = archauxv (libc :: AT_HWCAP) ; let hwcap2 = archauxv (libc :: AT_HWCAP2) ; if hwcap != 0 || hwcap2 != 0 { return Ok (AuxVec { hwcap , hwcap2 }) ; } Err (()) }
}

macro_rules! archauxv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function archauxv in module {}", module_path!());
    };
}

mkfn!{
    archauxv_introspect!();
    # [doc = " Tries to read the `key` from the auxiliary vector."] fn archauxv (key : libc :: c_int) -> usize { const OUT_LEN : libc :: c_int = core :: mem :: size_of :: < libc :: c_ulong > () as libc :: c_int ; let mut out : libc :: c_ulong = 0 ; unsafe { let res = libc :: elf_aux_info (key , & mut out as * mut libc :: c_ulong as * mut libc :: c_void , OUT_LEN) ; debug_assert ! (res == 0 || out == 0) ; } out as usize }
}