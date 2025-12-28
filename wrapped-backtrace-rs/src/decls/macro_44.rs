macro_rules! macro_44 {
    () => {
        cfg_if :: cfg_if ! { if # [cfg (feature = "std")] { pub use self :: backtrace :: trace ; pub use self :: symbolize :: { resolve , resolve_frame } ; pub use self :: capture :: { Backtrace , BacktraceFrame , BacktraceSymbol } ; mod capture ; } }
    };
}

macro_44!()