macro_rules! ioctl {
    () => {
        # [cfg (any (bsd , linux_android , solarish , target_os = "fuchsia" , target_os = "redox" ,))] # [cfg (feature = "ioctl")] # [cfg_attr (docsrs , doc (cfg (feature = "ioctl")))] # [macro_use] pub mod ioctl ;
    };
}

ioctl!();