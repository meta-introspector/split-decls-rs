macro_rules! Expectation {
    () => {
        # [doc = " When type-checking an expression, we propagate downward"] # [doc = " whatever type hint we are able in the form of an `Expectation`."] # [derive (Copy , Clone , Debug)] pub (crate) enum Expectation < 'tcx > { # [doc = " We know nothing about what type this expression should have."] NoExpectation , # [doc = " This expression should have the type given (or some subtype)."] ExpectHasType (Ty < 'tcx >) , # [doc = " This expression will be cast to the `Ty`."] ExpectCastableToType (Ty < 'tcx >) , # [doc = " This rvalue expression will be wrapped in `&` or `Box` and coerced"] # [doc = " to `&Ty` or `Box<Ty>`, respectively. `Ty` is `[A]` or `Trait`."] ExpectRvalueLikeUnsized (Ty < 'tcx >) , }
    };
}

Expectation!()