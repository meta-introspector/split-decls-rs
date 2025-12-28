macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! ModSpans {
    () => {
        deps!();
        # [derive (Copy , Clone , Encodable , Decodable , Debug , Default , Walkable)] pub struct ModSpans { # [doc = " `inner_span` covers the body of the module; for a file module, its the whole file."] # [doc = " For an inline module, its the span inside the `{ ... }`, not including the curly braces."] pub inner_span : Span , pub inject_use_span : Span , }
    };
}

ModSpans!()