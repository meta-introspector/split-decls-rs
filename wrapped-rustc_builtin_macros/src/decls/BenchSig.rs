macro_rules! BenchSig {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_bench_sig)] pub (crate) struct BenchSig { # [primary_span] pub (crate) span : Span , }
    };
}

BenchSig!();