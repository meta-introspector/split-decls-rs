mkuse!{pub (super) use fpu_precision :: set_precision ;}
mkmod!{fpu_precision, { 
                getname!(fpu_precision);
                getsrc!(fpu_precision);
                getpath!(fpu_precision);
                get_deps!(fpu_precision);
                get_crates!(fpu_precision);
                mkinclude!(fpu_precision);
                mkuse!{use core :: arch :: asm ;}
mkitem!{mkstruct!{# [doc = " A structure used to preserve the original value of the FPU control word, so that it can be"] # [doc = " restored when the structure is dropped."] # [doc = ""] # [doc = " The x87 FPU is a 16-bits register whose fields are as follows:"] # [doc = ""] # [doc = " | 12-15 | 10-11 | 8-9 | 6-7 |  5 |  4 |  3 |  2 |  1 |  0 |"] # [doc = " |------:|------:|----:|----:|---:|---:|---:|---:|---:|---:|"] # [doc = " |       | RC    | PC  |     | PM | UM | OM | ZM | DM | IM |"] # [doc = ""] # [doc = " The documentation for all of the fields is available in the IA-32 Architectures Software"] # [doc = " Developer's Manual (Volume 1)."] # [doc = ""] # [doc = " The only field which is relevant for the following code is PC, Precision Control. This"] # [doc = " field determines the precision of the operations performed by the FPU. It can be set to:"] # [doc = "  - 0b00, single precision i.e., 32-bits"] # [doc = "  - 0b10, double precision i.e., 64-bits"] # [doc = "  - 0b11, double extended precision i.e., 80-bits (default state)"] # [doc = " The 0b01 value is reserved and should not be used."] pub (crate) struct FPUControlWord (u16) ;}}

macro_rules! set_cw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_cw in module {}", module_path!());
    };
}

mkfn!{
    set_cw_introspect!();
    fn set_cw (cw : u16) { unsafe { asm ! ("fldcw word ptr [{}]" , in (reg) & cw , options (nostack) ,) } }
}

macro_rules! set_precision_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_precision in module {}", module_path!());
    };
}

mkfn!{
    set_precision_introspect!();
    # [doc = " Sets the precision field of the FPU to `T` and returns a `FPUControlWord`."] pub (crate) fn set_precision < T > () -> FPUControlWord { let mut cw = 0_u16 ; let cw_precision = match size_of :: < T > () { 4 => 0x0000 , 8 => 0x0200 , _ => 0x0300 , } ; unsafe { asm ! ("fnstcw word ptr [{}]" , in (reg) & mut cw , options (nostack) ,) } set_cw ((cw & 0xFCFF) | cw_precision) ; FPUControlWord (cw) }
}
mkitem!{mkimpl!{impl Drop for FPUControlWord { fn drop (& mut self) { set_cw (self . 0) } }}} 
            }}
mkmod!{fpu_precision, { 
                getname!(fpu_precision);
                getsrc!(fpu_precision);
                getpath!(fpu_precision);
                get_deps!(fpu_precision);
                get_crates!(fpu_precision);
                mkinclude!(fpu_precision);
                
macro_rules! set_precision_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_precision in module {}", module_path!());
    };
}

mkfn!{
    set_precision_introspect!();
    pub (crate) fn set_precision < T > () { }
} 
            }}