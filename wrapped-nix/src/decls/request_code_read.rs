macro_rules! request_code_read {
    () => {
        # [doc = " Generate an ioctl request code for a command that reads."] # [doc = ""] # [doc = " This is equivalent to the `_IOR()` macro exposed by the C ioctl API."] # [doc = ""] # [doc = " You should only use this macro directly if the `ioctl` you're working"] # [doc = " with is \"bad\" and you cannot use `ioctl_read!()` directly."] # [doc = ""] # [doc = " The read/write direction is relative to userland, so this"] # [doc = " command would be userland is reading and the kernel is"] # [doc = " writing."] # [macro_export (local_inner_macros)] macro_rules ! request_code_read { ($ g : expr , $ n : expr , $ len : expr) => { ioc ! ($ crate :: sys :: ioctl :: OUT , $ g , $ n , $ len) } ; }
    };
}

request_code_read!()