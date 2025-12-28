macro_rules! request_code_write {
    () => {
        # [doc = " Generate an ioctl request code for a command that writes."] # [doc = ""] # [doc = " This is equivalent to the `_IOW()` macro exposed by the C ioctl API."] # [doc = ""] # [doc = " You should only use this macro directly if the `ioctl` you're working"] # [doc = " with is \"bad\" and you cannot use `ioctl_write!()` directly."] # [doc = ""] # [doc = " The read/write direction is relative to userland, so this"] # [doc = " command would be userland is writing and the kernel is"] # [doc = " reading."] # [macro_export (local_inner_macros)] macro_rules ! request_code_write { ($ g : expr , $ n : expr , $ len : expr) => { ioc ! ($ crate :: sys :: ioctl :: IN , $ g , $ n , $ len) } ; }
    };
}

request_code_write!();