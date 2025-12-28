macro_rules! deps {
    () => {
        AsPrimitive!();
    };
}

macro_rules! AsCast {
    () => {
        deps!();
        # [doc = " An interface for casting between machine scalars."] pub trait AsCast : AsPrimitive { # [doc = " Creates a number from another value that can be converted into"] # [doc = " a primitive via the `AsPrimitive` trait."] fn as_cast < N : AsPrimitive > (n : N) -> Self ; }
    };
}

AsCast!()