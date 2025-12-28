macro_rules! DebugScope {
    () => {
        # [derive (Clone , Copy , Debug)] pub struct DebugScope < S , L > { pub dbg_scope : S , # [doc = " Call site location, if this scope was inlined from another function."] pub inlined_at : Option < L > , pub file_start_pos : BytePos , pub file_end_pos : BytePos , }
    };
}

DebugScope!()