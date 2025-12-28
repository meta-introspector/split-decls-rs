macro_rules! deps {
    () => {
        Float!();
        HFloat!();
    };
}

macro_rules! DFloat {
    () => {
        deps!();
        # [doc = " Trait for floats twice the bit width of another integer."] pub trait DFloat : Float { # [doc = " Float that is half the bit width of the floatthis trait is implemented for."] type H : HFloat < D = Self > ; # [doc = " Narrow the float type."] fn narrow (self) -> Self :: H ; }
    };
}

DFloat!()