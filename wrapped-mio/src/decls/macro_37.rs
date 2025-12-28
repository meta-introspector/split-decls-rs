macro_rules! macro_37 {
    () => {
        cfg_not_os_poll ! { mod shell ; pub (crate) use self :: shell ::*; # [cfg (unix)] cfg_any_os_ext ! { mod unix ; # [cfg (feature = "os-ext")] pub use self :: unix :: SourceFd ; } }
    };
}

macro_37!();