macro_rules! wants_new_eh_instructions {
    () => {
        # [doc = " Returns `true` if this session's target requires the new exception"] # [doc = " handling LLVM IR instructions (catchpad / cleanuppad / ... instead"] # [doc = " of landingpad)"] pub (crate) fn wants_new_eh_instructions (sess : & Session) -> bool { wants_wasm_eh (sess) || wants_msvc_seh (sess) }
    };
}

wants_new_eh_instructions!()