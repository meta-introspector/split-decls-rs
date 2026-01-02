mkuse!{use core :: arch :: asm ;}
mkuse!{use super :: super :: super :: generic ;}
mkuse!{use super :: detect :: { cpu_flags , get_cpu_features } ;}
mkuse!{use crate :: support :: Round ;}
mkuse!{use crate :: support :: feature_detect :: select_once ;}

macro_rules! fma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fma in module {}", module_path!());
    };
}

mkfn!{
    fma_introspect!();
    pub fn fma (x : f64 , y : f64 , z : f64) -> f64 { select_once ! { sig : fn (x : f64 , y : f64 , z : f64) -> f64 , init : || { let features = get_cpu_features () ; if features . contains (cpu_flags :: FMA) { fma_with_fma } else if features . contains (cpu_flags :: FMA4) { fma_with_fma4 } else { fma_fallback as Func } } , call : | fn_ptr : Func | unsafe { fn_ptr (x , y , z) } , } }
}

macro_rules! fmaf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaf in module {}", module_path!());
    };
}

mkfn!{
    fmaf_introspect!();
    pub fn fmaf (x : f32 , y : f32 , z : f32) -> f32 { select_once ! { sig : fn (x : f32 , y : f32 , z : f32) -> f32 , init : || { let features = get_cpu_features () ; if features . contains (cpu_flags :: FMA) { fmaf_with_fma } else if features . contains (cpu_flags :: FMA4) { fmaf_with_fma4 } else { fmaf_fallback as Func } } , call : | fn_ptr : Func | unsafe { fn_ptr (x , y , z) } , } }
}

macro_rules! fma_with_fma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fma_with_fma in module {}", module_path!());
    };
}

mkfn!{
    fma_with_fma_introspect!();
    # [doc = " # Safety"] # [doc = ""] # [doc = " Must have +fma available."] unsafe fn fma_with_fma (mut x : f64 , y : f64 , z : f64) -> f64 { debug_assert ! (get_cpu_features () . contains (cpu_flags :: FMA)) ; unsafe { asm ! ("vfmadd213sd {x}, {y}, {z}" , x = inout (xmm_reg) x , y = in (xmm_reg) y , z = in (xmm_reg) z , options (nostack , nomem , pure) ,) ; } x }
}

macro_rules! fmaf_with_fma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaf_with_fma in module {}", module_path!());
    };
}

mkfn!{
    fmaf_with_fma_introspect!();
    # [doc = " # Safety"] # [doc = ""] # [doc = " Must have +fma available."] unsafe fn fmaf_with_fma (mut x : f32 , y : f32 , z : f32) -> f32 { debug_assert ! (get_cpu_features () . contains (cpu_flags :: FMA)) ; unsafe { asm ! ("vfmadd213ss {x}, {y}, {z}" , x = inout (xmm_reg) x , y = in (xmm_reg) y , z = in (xmm_reg) z , options (nostack , nomem , pure) ,) ; } x }
}

macro_rules! fma_with_fma4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fma_with_fma4 in module {}", module_path!());
    };
}

mkfn!{
    fma_with_fma4_introspect!();
    # [doc = " # Safety"] # [doc = ""] # [doc = " Must have +fma4 available."] unsafe fn fma_with_fma4 (mut x : f64 , y : f64 , z : f64) -> f64 { debug_assert ! (get_cpu_features () . contains (cpu_flags :: FMA4)) ; unsafe { asm ! ("vfmaddsd {x}, {x}, {y}, {z}" , x = inout (xmm_reg) x , y = in (xmm_reg) y , z = in (xmm_reg) z , options (nostack , nomem , pure) ,) ; } x }
}

macro_rules! fmaf_with_fma4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaf_with_fma4 in module {}", module_path!());
    };
}

mkfn!{
    fmaf_with_fma4_introspect!();
    # [doc = " # Safety"] # [doc = ""] # [doc = " Must have +fma4 available."] unsafe fn fmaf_with_fma4 (mut x : f32 , y : f32 , z : f32) -> f32 { debug_assert ! (get_cpu_features () . contains (cpu_flags :: FMA4)) ; unsafe { asm ! ("vfmaddss {x}, {x}, {y}, {z}" , x = inout (xmm_reg) x , y = in (xmm_reg) y , z = in (xmm_reg) z , options (nostack , nomem , pure) ,) ; } x }
}

macro_rules! fma_fallback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fma_fallback in module {}", module_path!());
    };
}

mkfn!{
    fma_fallback_introspect!();
    fn fma_fallback (x : f64 , y : f64 , z : f64) -> f64 { generic :: fma_round (x , y , z , Round :: Nearest) . val }
}

macro_rules! fmaf_fallback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaf_fallback in module {}", module_path!());
    };
}

mkfn!{
    fmaf_fallback_introspect!();
    fn fmaf_fallback (x : f32 , y : f32 , z : f32) -> f32 { generic :: fma_wide_round (x , y , z , Round :: Nearest) . val }
}