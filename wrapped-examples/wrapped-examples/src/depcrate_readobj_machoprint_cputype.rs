// Generated macro for print_cputype (function)
macro_rules! Depcrate_readobj_machoprint_cputype {
() => {
// Module: crate::readobj::macho
// Provides: {"print_cputype"}
// Dependencies: {}
fn print_cputype (p : & mut Printer < '_ > , cputype : u32 , cpusubtype : u32) { let proc = match cputype { CPU_TYPE_ANY => FLAGS_CPU_SUBTYPE_ANY , CPU_TYPE_VAX => FLAGS_CPU_SUBTYPE_VAX , CPU_TYPE_MC680X0 => FLAGS_CPU_SUBTYPE_MC680X0 , CPU_TYPE_X86 => FLAGS_CPU_SUBTYPE_X86 , CPU_TYPE_X86_64 => FLAGS_CPU_SUBTYPE_X86_64 , CPU_TYPE_MIPS => FLAGS_CPU_SUBTYPE_MIPS , CPU_TYPE_MC98000 => FLAGS_CPU_SUBTYPE_MC98000 , CPU_TYPE_HPPA => FLAGS_CPU_SUBTYPE_HPPA , CPU_TYPE_ARM => FLAGS_CPU_SUBTYPE_ARM , CPU_TYPE_ARM64 => FLAGS_CPU_SUBTYPE_ARM64 , CPU_TYPE_ARM64_32 => FLAGS_CPU_SUBTYPE_ARM64_32 , CPU_TYPE_MC88000 => FLAGS_CPU_SUBTYPE_MC88000 , CPU_TYPE_SPARC => FLAGS_CPU_SUBTYPE_SPARC , CPU_TYPE_I860 => FLAGS_CPU_SUBTYPE_I860 , CPU_TYPE_POWERPC | CPU_TYPE_POWERPC64 => FLAGS_CPU_SUBTYPE_POWERPC , _ => & [] , } ; p . field_enum ("CpuType" , cputype , FLAGS_CPU_TYPE) ; p . field_hex ("CpuSubtype" , cpusubtype) ; p . flags (cpusubtype , ! CPU_SUBTYPE_MASK , proc) ; p . flags (cpusubtype , 0 , FLAGS_CPU_SUBTYPE) ; }
};
}
