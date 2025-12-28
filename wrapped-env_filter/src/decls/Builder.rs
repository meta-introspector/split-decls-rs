macro_rules! deps {
    () => {
        Directive!();
        Filter!();
        FilterOp!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " A builder for a log filter."] # [doc = ""] # [doc = " It can be used to parse a set of directives from a string before building"] # [doc = " a [`Filter`] instance."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::env;"] # [doc = " use env_filter::Builder;"] # [doc = ""] # [doc = " let mut builder = Builder::new();"] # [doc = ""] # [doc = " // Parse a logging filter from an environment variable."] # [doc = " if let Ok(rust_log) = env::var(\"RUST_LOG\") {"] # [doc = "     builder.parse(&rust_log);"] # [doc = " }"] # [doc = ""] # [doc = " let filter = builder.build();"] # [doc = " ```"] pub struct Builder { directives : Vec < Directive > , filter : Option < FilterOp > , built : bool , }
    };
}

Builder!()