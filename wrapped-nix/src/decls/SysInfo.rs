macro_rules! SysInfo {
    () => {
        # [doc = " System info structure returned by `sysinfo`."] # [derive (Copy , Clone , Debug , Eq , Hash , PartialEq)] # [repr (transparent)] pub struct SysInfo (libc :: sysinfo) ;
    };
}

SysInfo!()