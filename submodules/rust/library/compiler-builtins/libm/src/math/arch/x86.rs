mkmod!{detect, { 
                getname!(detect);
                getsrc!(detect);
                getpath!(detect);
                get_deps!(detect);
                get_crates!(detect);
                mkinclude!(detect);
                 
            }}
mkmod!{fma, { 
                getname!(fma);
                getsrc!(fma);
                getpath!(fma);
                get_deps!(fma);
                get_crates!(fma);
                mkinclude!(fma);
                 
            }}
mkuse!{pub use fma :: { fma , fmaf } ;}

macro_rules! sqrtf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sqrtf in module {}", module_path!());
    };
}

mkfn!{
    sqrtf_introspect!();
    pub fn sqrtf (mut x : f32) -> f32 { unsafe { core :: arch :: asm ! ("sqrtss {x}, {x}" , x = inout (xmm_reg) x , options (nostack , nomem , pure) ,) } ; x }
}

macro_rules! sqrt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sqrt in module {}", module_path!());
    };
}

mkfn!{
    sqrt_introspect!();
    pub fn sqrt (mut x : f64) -> f64 { unsafe { core :: arch :: asm ! ("sqrtsd {x}, {x}" , x = inout (xmm_reg) x , options (nostack , nomem , pure) ,) } ; x }
}