macro_rules! deps {
    () => {
        ForeignItem!();
        Walkable!();
        StrLit!();
        Safety!();
    };
}

macro_rules! ForeignMod {
    () => {
        deps!();
        # [doc = " Foreign module declaration."] # [doc = ""] # [doc = " E.g., `extern { .. }` or `extern \"C\" { .. }`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct ForeignMod { # [doc = " Span of the `extern` keyword."] pub extern_span : Span , # [doc = " `unsafe` keyword accepted syntactically for macro DSLs, but not"] # [doc = " semantically by Rust."] pub safety : Safety , pub abi : Option < StrLit > , pub items : ThinVec < Box < ForeignItem > > , }
    };
}

ForeignMod!();