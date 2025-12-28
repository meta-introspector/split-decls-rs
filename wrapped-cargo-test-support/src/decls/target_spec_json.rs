macro_rules! target_spec_json {
    () => {
        # [doc = " Gets a valid target spec JSON from rustc."] # [doc = ""] # [doc = " To avoid any hardcoded value, this fetches `x86_64-unknown-none` target"] # [doc = " spec JSON directly from `rustc`, as Cargo shouldn't know the JSON schema."] pub fn target_spec_json () -> & 'static str { static TARGET_SPEC_JSON : LazyLock < String > = LazyLock :: new (| | { let json = std :: process :: Command :: new ("rustc") . env ("RUSTC_BOOTSTRAP" , "1") . arg ("--print") . arg ("target-spec-json") . arg ("-Zunstable-options") . arg ("--target") . arg ("x86_64-unknown-none") . output () . expect ("rustc --print target-spec-json") . stdout ; String :: from_utf8 (json) . expect ("utf8 target spec json") }) ; TARGET_SPEC_JSON . as_str () }
    };
}

target_spec_json!()