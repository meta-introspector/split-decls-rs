macro_rules! deps {
    () => {
        DFloat!();
        Float!();
    };
}

macro_rules! HFloat {
    () => {
        deps!();
        # [doc = " Trait for floats half the bit width of another float."] pub trait HFloat : Float { # [doc = " Float that is double the bit width of the float this trait is implemented for."] type D : DFloat < H = Self > ; # [doc = " Widen the float type."] fn widen (self) -> Self :: D ; }
    };
}

HFloat!();