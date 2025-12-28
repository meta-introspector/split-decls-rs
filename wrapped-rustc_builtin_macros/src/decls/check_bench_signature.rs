macro_rules! deps {
    () => {
        BenchSig!();
    };
}

macro_rules! check_bench_signature {
    () => {
        deps!();
        fn check_bench_signature (cx : & ExtCtxt < '_ > , i : & ast :: Item , f : & ast :: Fn ,) -> Result < () , ErrorGuaranteed > { if f . sig . decl . inputs . len () != 1 { return Err (cx . dcx () . emit_err (errors :: BenchSig { span : i . span })) ; } Ok (()) }
    };
}

check_bench_signature!();