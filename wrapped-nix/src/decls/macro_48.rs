macro_rules! macro_48 {
    () => {
        feature ! { #! [feature = "net"] # [cfg (any (linux_android , bsd , solarish , target_os = "hurd"))] # [deny (missing_docs)] pub mod ifaddrs ; # [cfg (not (target_os = "redox"))] # [deny (missing_docs)] pub mod net ; }
    };
}

macro_48!()