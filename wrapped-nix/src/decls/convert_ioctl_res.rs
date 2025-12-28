macro_rules! convert_ioctl_res {
    () => {
        # [doc = " Convert raw ioctl return value to a Nix result"] # [macro_export] # [doc (hidden)] macro_rules ! convert_ioctl_res { ($ w : expr) => { { $ crate :: errno :: Errno :: result ($ w) } } ; }
    };
}

convert_ioctl_res!();