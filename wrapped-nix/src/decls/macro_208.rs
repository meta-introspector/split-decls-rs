macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! macro_208 {
    () => {
        deps!();
        feature ! { #! [all (feature = "process" , feature = "term")] # [doc = " Get the terminal foreground process group (see"] # [doc = " [tcgetpgrp(3)](https://pubs.opengroup.org/onlinepubs/9699919799/functions/tcgetpgrp.html))."] # [doc = ""] # [doc = " Get the group process id (GPID) of the foreground process group on the"] # [doc = " terminal associated to file descriptor (FD)."] # [inline] pub fn tcgetpgrp < F : std :: os :: fd :: AsFd > (fd : F) -> Result < Pid > { use std :: os :: fd :: AsRawFd ; let res = unsafe { libc :: tcgetpgrp (fd . as_fd () . as_raw_fd ()) } ; Errno :: result (res) . map (Pid) } # [doc = " Set the terminal foreground process group (see"] # [doc = " [tcgetpgrp(3)](https://pubs.opengroup.org/onlinepubs/9699919799/functions/tcsetpgrp.html))."] # [doc = ""] # [doc = " Get the group process id (PGID) to the foreground process group on the"] # [doc = " terminal associated to file descriptor (FD)."] # [inline] pub fn tcsetpgrp < F : std :: os :: fd :: AsFd > (fd : F , pgrp : Pid) -> Result < () > { use std :: os :: fd :: AsRawFd ; let res = unsafe { libc :: tcsetpgrp (fd . as_fd () . as_raw_fd () , pgrp . into ()) } ; Errno :: result (res) . map (drop) } }
    };
}

macro_208!()