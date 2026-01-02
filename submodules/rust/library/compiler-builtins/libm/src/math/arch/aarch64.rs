mkuse!{use core :: arch :: asm ;}

macro_rules! fma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fma in module {}", module_path!());
    };
}

mkfn!{
    fma_introspect!();
    pub fn fma (mut x : f64 , y : f64 , z : f64) -> f64 { unsafe { asm ! ("fmadd {x:d}, {x:d}, {y:d}, {z:d}" , x = inout (vreg) x , y = in (vreg) y , z = in (vreg) z , options (nomem , nostack , pure)) ; } x }
}

macro_rules! fmaf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaf in module {}", module_path!());
    };
}

mkfn!{
    fmaf_introspect!();
    pub fn fmaf (mut x : f32 , y : f32 , z : f32) -> f32 { unsafe { asm ! ("fmadd {x:s}, {x:s}, {y:s}, {z:s}" , x = inout (vreg) x , y = in (vreg) y , z = in (vreg) z , options (nomem , nostack , pure)) ; } x }
}

macro_rules! rint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rint in module {}", module_path!());
    };
}

mkfn!{
    rint_introspect!();
    pub fn rint (mut x : f64) -> f64 { unsafe { asm ! ("frintn {x:d}, {x:d}" , x = inout (vreg) x , options (nomem , nostack , pure)) ; } x }
}

macro_rules! rintf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rintf in module {}", module_path!());
    };
}

mkfn!{
    rintf_introspect!();
    pub fn rintf (mut x : f32) -> f32 { unsafe { asm ! ("frintn {x:s}, {x:s}" , x = inout (vreg) x , options (nomem , nostack , pure)) ; } x }
}

macro_rules! rintf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rintf16 in module {}", module_path!());
    };
}

mkfn!{
    rintf16_introspect!();
    # [cfg (all (f16_enabled , target_feature = "fp16"))] pub fn rintf16 (mut x : f16) -> f16 { unsafe { asm ! ("frintn {x:h}, {x:h}" , x = inout (vreg) x , options (nomem , nostack , pure)) ; } x }
}

macro_rules! sqrt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sqrt in module {}", module_path!());
    };
}

mkfn!{
    sqrt_introspect!();
    pub fn sqrt (mut x : f64) -> f64 { unsafe { asm ! ("fsqrt {x:d}, {x:d}" , x = inout (vreg) x , options (nomem , nostack , pure)) ; } x }
}

macro_rules! sqrtf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sqrtf in module {}", module_path!());
    };
}

mkfn!{
    sqrtf_introspect!();
    pub fn sqrtf (mut x : f32) -> f32 { unsafe { asm ! ("fsqrt {x:s}, {x:s}" , x = inout (vreg) x , options (nomem , nostack , pure)) ; } x }
}

macro_rules! sqrtf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sqrtf16 in module {}", module_path!());
    };
}

mkfn!{
    sqrtf16_introspect!();
    # [cfg (all (f16_enabled , target_feature = "fp16"))] pub fn sqrtf16 (mut x : f16) -> f16 { unsafe { asm ! ("fsqrt {x:h}, {x:h}" , x = inout (vreg) x , options (nomem , nostack , pure)) ; } x }
}