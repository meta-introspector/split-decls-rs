macro_rules! alternate_arch {
    () => {
        pub fn alternate_arch () -> & 'static str { if cfg ! (target_os = "macos") { "x86_64" } else { "x86" } }
    };
}

alternate_arch!();