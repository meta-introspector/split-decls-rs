macro_rules! sleep {
    () => {
        # [doc = " Suspend execution for an interval of time"] # [doc = ""] # [doc = " See also [sleep(2)](https://pubs.opengroup.org/onlinepubs/009695399/functions/sleep.html#tag_03_705_05)"] # [inline] pub fn sleep (seconds : c_uint) -> c_uint { unsafe { libc :: sleep (seconds) } }
    };
}

sleep!();