macro_rules! Sp {
    () => {
        # [doc = " An entity with a span attached."] # [derive (Debug , Copy , Clone)] pub (crate) struct Sp < T > { val : T , span : Span , }
    };
}

Sp!()