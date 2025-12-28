macro_rules! deps {
    () => {
        ObjectType!();
    };
}

macro_rules! CastOrPanic {
    () => {
        deps!();
        # [doc = " This trait is useful to export cast_or_panic into crate but not outside"] pub trait CastOrPanic { fn cast_or_panic < T > (self , kind : ObjectType) -> T ; }
    };
}

CastOrPanic!()