macro_rules! macro_34 {
    () => {
        # [cfg (any (unix , target_os = "hermit"))] cfg_os_poll ! { mod unix ; # [allow (unused_imports)] pub use self :: unix ::*; }
    };
}

macro_34!();