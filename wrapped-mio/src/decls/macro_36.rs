macro_rules! macro_36 {
    () => {
        # [cfg (target_os = "wasi")] cfg_os_poll ! { mod wasi ; pub (crate) use self :: wasi ::*; }
    };
}

macro_36!()