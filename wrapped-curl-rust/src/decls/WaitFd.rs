macro_rules! WaitFd {
    () => {
        # [doc = " File descriptor to wait on for use with the `wait` method on a multi handle."] pub struct WaitFd { inner : curl_sys :: curl_waitfd , }
    };
}

WaitFd!()