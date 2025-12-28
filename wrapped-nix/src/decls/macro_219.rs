macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! macro_219 {
    () => {
        deps!();
        feature ! { #! [feature = "fs"] # [doc = " Directive that tells [`lseek`] and [`lseek64`] what the offset is relative to."] # [doc = ""] # [doc = " [`lseek`]: ./fn.lseek.html"] # [doc = " [`lseek64`]: ./fn.lseek64.html"] # [repr (i32)] # [derive (Clone , Copy , Debug)] pub enum Whence { # [doc = " Specify an offset relative to the start of the file."] SeekSet = libc :: SEEK_SET , # [doc = " Specify an offset relative to the current file location."] SeekCur = libc :: SEEK_CUR , # [doc = " Specify an offset relative to the end of the file."] SeekEnd = libc :: SEEK_END , # [doc = " Specify an offset relative to the next location in the file greater than or"] # [doc = " equal to offset that contains some data. If offset points to"] # [doc = " some data, then the file offset is set to offset."] # [cfg (any (apple_targets , freebsdlike , solarish , target_os = "hurd" , target_os = "linux" ,))] SeekData = libc :: SEEK_DATA , # [doc = " Specify an offset relative to the next hole in the file greater than"] # [doc = " or equal to offset. If offset points into the middle of a hole, then"] # [doc = " the file offset should be set to offset. If there is no hole past offset,"] # [doc = " then the file offset should be adjusted to the end of the file (i.e., there"] # [doc = " is an implicit hole at the end of any file)."] # [cfg (any (apple_targets , freebsdlike , solarish , target_os = "hurd" , target_os = "linux" ,))] SeekHole = libc :: SEEK_HOLE , } # [doc = " Move the read/write file offset."] # [doc = ""] # [doc = " See also [lseek(2)](https://pubs.opengroup.org/onlinepubs/9699919799/functions/lseek.html)"] pub fn lseek < Fd : std :: os :: fd :: AsFd > (fd : Fd , offset : off_t , whence : Whence) -> Result < off_t > { use std :: os :: fd :: AsRawFd ; let res = unsafe { libc :: lseek (fd . as_fd () . as_raw_fd () , offset , whence as i32) } ; Errno :: result (res) . map (| r | r as off_t) } # [doc = " Move the read/write file offset."] # [doc = ""] # [doc = " Unlike [`lseek`], it takes a 64-bit argument even on platforms where [`libc::off_t`] is"] # [doc = " 32 bits."] # [cfg (linux_android)] pub fn lseek64 < Fd : std :: os :: fd :: AsFd > (fd : Fd , offset : libc :: off64_t , whence : Whence ,) -> Result < libc :: off64_t > { use std :: os :: fd :: AsRawFd ; let res = unsafe { libc :: lseek64 (fd . as_fd () . as_raw_fd () , offset , whence as i32) } ; Errno :: result (res) . map (| r | r as libc :: off64_t) } }
    };
}

macro_219!();