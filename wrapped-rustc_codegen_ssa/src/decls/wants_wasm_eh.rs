macro_rules! wants_wasm_eh {
    () => {
        pub fn wants_wasm_eh (sess : & Session) -> bool { sess . target . is_like_wasm && (sess . target . os != "emscripten" || sess . opts . unstable_opts . emscripten_wasm_eh) }
    };
}

wants_wasm_eh!();