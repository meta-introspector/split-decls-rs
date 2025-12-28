macro_rules! deps {
    () => {
        BoundedMeasure!();
    };
}

macro_rules! impl_bounded_measure_integer {
    () => {
        deps!();
        macro_rules ! impl_bounded_measure_integer (($ ($ t : ident) ,*) => { $ (impl BoundedMeasure for $ t { fn min () -> Self { $ t :: MIN } fn max () -> Self { $ t :: MAX } fn overflowing_add (self , rhs : Self) -> (Self , bool) { self . overflowing_add (rhs) } fn from_f32 (val : f32) -> Self { val as $ t } fn from_f64 (val : f64) -> Self { val as $ t } }) * } ;) ;
    };
}

impl_bounded_measure_integer!()