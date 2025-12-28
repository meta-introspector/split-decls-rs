macro_rules! deps {
    () => {
        LinuxInfo!();
    };
}

macro_rules! LinuxDetails {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub enum LinuxDetails { Info (LinuxInfo) , Error (String) , Unknown , }
    };
}

LinuxDetails!();