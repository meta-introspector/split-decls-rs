macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! close {
    () => {
        deps!();
        # [doc = " Close a file descriptor."] # [doc = ""] # [doc = " If `fd` is an owned file descriptor, it is generally preferred to call"] # [doc = " `drop(fd)` rather than `close(fd)`."] pub fn close < Fd : std :: os :: fd :: IntoRawFd > (fd : Fd) -> Result < () > { let res = unsafe { libc :: close (fd . into_raw_fd ()) } ; Errno :: result (res) . map (drop) }
    };
}

close!()