macro_rules! deps {
    () => {
        DebugStrictAdd!();
    };
}

macro_rules! impl_debug_strict_add {
    () => {
        deps!();
        macro_rules ! impl_debug_strict_add { ($ ($ ty : ty) *) => { $ (impl DebugStrictAdd for $ ty { fn debug_strict_add (self , other : Self) -> Self { if cfg ! (debug_assertions) { self + other } else { self . wrapping_add (other) } } }) * } ; }
    };
}

impl_debug_strict_add!()