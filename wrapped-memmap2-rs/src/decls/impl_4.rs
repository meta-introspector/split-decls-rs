macro_rules! deps {
    () => {
        UncheckedAdvice!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        # [cfg (target_os = "linux")] impl UncheckedAdvice { # [doc = " Performs a runtime check if this advice is supported by the kernel."] # [doc = " Only supported on Linux. See the [`madvise(2)`] man page."] # [doc = ""] # [doc = " [`madvise(2)`]: https://man7.org/linux/man-pages/man2/madvise.2.html#VERSIONS"] pub fn is_supported (self) -> bool { (unsafe { libc :: madvise (std :: ptr :: null_mut () , 0 , self as libc :: c_int) }) == 0 } }
    };
}

impl_4!();