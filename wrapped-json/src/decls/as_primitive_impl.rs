macro_rules! deps {
    () => {
        AsPrimitive!();
    };
}

macro_rules! as_primitive_impl {
    () => {
        deps!();
        macro_rules ! as_primitive_impl { ($ ($ ty : ident) *) => { $ (impl AsPrimitive for $ ty { # [inline] fn as_u32 (self) -> u32 { self as u32 } # [inline] fn as_u64 (self) -> u64 { self as u64 } # [inline] fn as_u128 (self) -> u128 { self as u128 } # [inline] fn as_usize (self) -> usize { self as usize } # [inline] fn as_f32 (self) -> f32 { self as f32 } # [inline] fn as_f64 (self) -> f64 { self as f64 } }) * } ; }
    };
}

as_primitive_impl!()