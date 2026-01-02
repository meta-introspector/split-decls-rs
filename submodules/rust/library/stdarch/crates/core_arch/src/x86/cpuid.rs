mkuse!{use crate :: arch :: asm ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{mkstruct!{# [doc = " Result of the `cpuid` instruction."] # [allow (clippy :: missing_inline_in_public_items)] # [derive (Copy , Clone , Debug , Eq , Ord , PartialEq , PartialOrd)] # [stable (feature = "simd_x86" , since = "1.27.0")] pub struct CpuidResult { # [doc = " EAX register."] # [stable (feature = "simd_x86" , since = "1.27.0")] pub eax : u32 , # [doc = " EBX register."] # [stable (feature = "simd_x86" , since = "1.27.0")] pub ebx : u32 , # [doc = " ECX register."] # [stable (feature = "simd_x86" , since = "1.27.0")] pub ecx : u32 , # [doc = " EDX register."] # [stable (feature = "simd_x86" , since = "1.27.0")] pub edx : u32 , }}}

macro_rules! __cpuid_count_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __cpuid_count in module {}", module_path!());
    };
}

mkfn!{
    __cpuid_count_introspect!();
    # [doc = " Returns the result of the `cpuid` instruction for a given `leaf` (`EAX`)"] # [doc = " and `sub_leaf` (`ECX`)."] # [doc = ""] # [doc = " The highest-supported leaf value is returned by the first tuple argument of"] # [doc = " [`__get_cpuid_max(0)`](fn.__get_cpuid_max.html). For leaves containing"] # [doc = " sub-leaves, the second tuple argument returns the highest-supported"] # [doc = " sub-leaf value."] # [doc = ""] # [doc = " The [CPUID Wikipedia page][wiki_cpuid] contains how to query which"] # [doc = " information using the `EAX` and `ECX` registers, and the interpretation of"] # [doc = " the results returned in `EAX`, `EBX`, `ECX`, and `EDX`."] # [doc = ""] # [doc = " The references are:"] # [doc = " - [Intel 64 and IA-32 Architectures Software Developer's Manual Volume 2:"] # [doc = "   Instruction Set Reference, A-Z][intel64_ref]."] # [doc = " - [AMD64 Architecture Programmer's Manual, Volume 3: General-Purpose and"] # [doc = "   System Instructions][amd64_ref]."] # [doc = ""] # [doc = " [wiki_cpuid]: https://en.wikipedia.org/wiki/CPUID"] # [doc = " [intel64_ref]: https://cdrdv2-public.intel.com/671110/325383-sdm-vol-2abcd.pdf"] # [doc = " [amd64_ref]: http://support.amd.com/TechDocs/24594.pdf"] # [inline] # [cfg_attr (test , assert_instr (cpuid))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn __cpuid_count (leaf : u32 , sub_leaf : u32) -> CpuidResult { let eax ; let ebx ; let ecx ; let edx ; # [cfg (target_arch = "x86")] { asm ! ("mov {0}, ebx" , "cpuid" , "xchg {0}, ebx" , out (reg) ebx , inout ("eax") leaf => eax , inout ("ecx") sub_leaf => ecx , out ("edx") edx , options (nostack , preserves_flags) ,) ; } # [cfg (target_arch = "x86_64")] { asm ! ("mov {0:r}, rbx" , "cpuid" , "xchg {0:r}, rbx" , out (reg) ebx , inout ("eax") leaf => eax , inout ("ecx") sub_leaf => ecx , out ("edx") edx , options (nostack , preserves_flags) ,) ; } CpuidResult { eax , ebx , ecx , edx } }
}

macro_rules! __cpuid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __cpuid in module {}", module_path!());
    };
}

mkfn!{
    __cpuid_introspect!();
    # [doc = " See [`__cpuid_count`](fn.__cpuid_count.html)."] # [inline] # [cfg_attr (test , assert_instr (cpuid))] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn __cpuid (leaf : u32) -> CpuidResult { __cpuid_count (leaf , 0) }
}

macro_rules! __get_cpuid_max_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __get_cpuid_max in module {}", module_path!());
    };
}

mkfn!{
    __get_cpuid_max_introspect!();
    # [doc = " Returns the highest-supported `leaf` (`EAX`) and sub-leaf (`ECX`) `cpuid`"] # [doc = " values."] # [doc = ""] # [doc = " If `cpuid` is supported, and `leaf` is zero, then the first tuple argument"] # [doc = " contains the highest `leaf` value that `cpuid` supports. For `leaf`s"] # [doc = " containing sub-leafs, the second tuple argument contains the"] # [doc = " highest-supported sub-leaf value."] # [doc = ""] # [doc = " See also [`__cpuid`](fn.__cpuid.html) and"] # [doc = " [`__cpuid_count`](fn.__cpuid_count.html)."] # [inline] # [stable (feature = "simd_x86" , since = "1.27.0")] pub unsafe fn __get_cpuid_max (leaf : u32) -> (u32 , u32) { let CpuidResult { eax , ebx , .. } = __cpuid (leaf) ; (eax , ebx) }
}