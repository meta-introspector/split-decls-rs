macro_rules! macro_26 {
    () => {
        cfg_default ! { # [cfg (not (target_os = "unknown"))] pub mod fs ; pub mod path ; pub mod net ; # [cfg (not (target_os = "unknown"))] pub (crate) mod rt ; }
    };
}

macro_26!()