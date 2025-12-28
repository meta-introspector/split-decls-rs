macro_rules! deps {
    () => {
        Callee!();
        Type!();
    };
}

macro_rules! Callable {
    () => {
        deps!();
        # [derive (Debug)] pub struct Callable < 'db > { ty : Type < 'db > , sig : PolyFnSig < 'db > , callee : Callee < 'db > , # [doc = " Whether this is a method that was called with method call syntax."] is_bound_method : bool , }
    };
}

Callable!()