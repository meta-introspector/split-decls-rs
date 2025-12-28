macro_rules! unused {
    () => {
        # [doc = " A target-triple that is neither the host nor the target."] # [doc = ""] # [doc = " Rustc may not work with it and it's alright, apart from being a"] # [doc = " valid target triple it is supposed to be used only as a"] # [doc = " placeholder for targets that should not be considered."] pub fn unused () -> & 'static str { "wasm32-unknown-unknown" }
    };
}

unused!()