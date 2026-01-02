
macro_rules! ceil_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ceil in module {}", module_path!());
    };
}

mkfn!{
    ceil_introspect!();
    pub fn ceil (x : f64) -> f64 { core :: arch :: wasm32 :: f64_ceil (x) }
}

macro_rules! ceilf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ceilf in module {}", module_path!());
    };
}

mkfn!{
    ceilf_introspect!();
    pub fn ceilf (x : f32) -> f32 { core :: arch :: wasm32 :: f32_ceil (x) }
}

macro_rules! fabs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fabs in module {}", module_path!());
    };
}

mkfn!{
    fabs_introspect!();
    pub fn fabs (x : f64) -> f64 { x . abs () }
}

macro_rules! fabsf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fabsf in module {}", module_path!());
    };
}

mkfn!{
    fabsf_introspect!();
    pub fn fabsf (x : f32) -> f32 { x . abs () }
}

macro_rules! floor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function floor in module {}", module_path!());
    };
}

mkfn!{
    floor_introspect!();
    pub fn floor (x : f64) -> f64 { core :: arch :: wasm32 :: f64_floor (x) }
}

macro_rules! floorf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function floorf in module {}", module_path!());
    };
}

mkfn!{
    floorf_introspect!();
    pub fn floorf (x : f32) -> f32 { core :: arch :: wasm32 :: f32_floor (x) }
}

macro_rules! rint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rint in module {}", module_path!());
    };
}

mkfn!{
    rint_introspect!();
    pub fn rint (x : f64) -> f64 { core :: arch :: wasm32 :: f64_nearest (x) }
}

macro_rules! rintf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rintf in module {}", module_path!());
    };
}

mkfn!{
    rintf_introspect!();
    pub fn rintf (x : f32) -> f32 { core :: arch :: wasm32 :: f32_nearest (x) }
}

macro_rules! sqrt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sqrt in module {}", module_path!());
    };
}

mkfn!{
    sqrt_introspect!();
    pub fn sqrt (x : f64) -> f64 { core :: arch :: wasm32 :: f64_sqrt (x) }
}

macro_rules! sqrtf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sqrtf in module {}", module_path!());
    };
}

mkfn!{
    sqrtf_introspect!();
    pub fn sqrtf (x : f32) -> f32 { core :: arch :: wasm32 :: f32_sqrt (x) }
}

macro_rules! trunc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trunc in module {}", module_path!());
    };
}

mkfn!{
    trunc_introspect!();
    pub fn trunc (x : f64) -> f64 { core :: arch :: wasm32 :: f64_trunc (x) }
}

macro_rules! truncf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function truncf in module {}", module_path!());
    };
}

mkfn!{
    truncf_introspect!();
    pub fn truncf (x : f32) -> f32 { core :: arch :: wasm32 :: f32_trunc (x) }
}