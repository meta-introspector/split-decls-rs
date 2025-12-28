macro_rules! BoolLit {
    () => {
        # [doc = " A bool literal: `true` or `false`. Also see [the reference][ref]."] # [doc = ""] # [doc = " Notice that, strictly speaking, from Rust point of view \"boolean literals\" are not"] # [doc = " actual literals but [keywords]."] # [doc = ""] # [doc = " [ref]: https://doc.rust-lang.org/reference/expressions/literal-expr.html#boolean-literal-expressions"] # [doc = " [keywords]: https://doc.rust-lang.org/reference/keywords.html#strict-keywords"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum BoolLit { False , True , }
    };
}

BoolLit!();