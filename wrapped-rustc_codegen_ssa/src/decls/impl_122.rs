macro_rules! deps {
    () => {
        Command!();
        WasmLd!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < 'a > WasmLd < 'a > { fn new (cmd : Command , sess : & 'a Session) -> WasmLd < 'a > { let mut wasm_ld = WasmLd { cmd , sess } ; if sess . target_features . contains (& sym :: atomics) { wasm_ld . link_args (& ["--shared-memory" , "--max-memory=1073741824" , "--import-memory"]) ; if sess . target . os == "unknown" || sess . target . os == "none" { wasm_ld . link_args (& ["--export=__wasm_init_tls" , "--export=__tls_size" , "--export=__tls_align" , "--export=__tls_base" ,]) ; } } wasm_ld } }
    };
}

impl_122!()