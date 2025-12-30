// Generated macro for Builder (struct)
macro_rules! Depcrate_packed_teddy_builderBuilder {
() => {
// Module: crate::packed::teddy::builder
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A builder for constructing a Teddy matcher."] # [doc = ""] # [doc = " The builder primarily permits fine grained configuration of the Teddy"] # [doc = " matcher. Most options are made only available for testing/benchmarking"] # [doc = " purposes. In reality, options are automatically determined by the nature"] # [doc = " and number of patterns given to the builder."] # [derive (Clone , Debug)] pub (crate) struct Builder { # [doc = " When none, this is automatically determined. Otherwise, `false` means"] # [doc = " slim Teddy is used (8 buckets) and `true` means fat Teddy is used"] # [doc = " (16 buckets). Fat Teddy requires AVX2, so if that CPU feature isn't"] # [doc = " available and Fat Teddy was requested, no matcher will be built."] only_fat : Option < bool > , # [doc = " When none, this is automatically determined. Otherwise, `false` means"] # [doc = " that 128-bit vectors will be used (up to SSSE3 instructions) where as"] # [doc = " `true` means that 256-bit vectors will be used. As with `fat`, if"] # [doc = " 256-bit vectors are requested and they aren't available, then a"] # [doc = " searcher will not be built."] only_256bit : Option < bool > , # [doc = " When true (the default), the number of patterns will be used as a"] # [doc = " heuristic for refusing construction of a Teddy searcher. The point here"] # [doc = " is that too many patterns can overwhelm Teddy. But this can be disabled"] # [doc = " in cases where the caller knows better."] heuristic_pattern_limits : bool , }
};
}
