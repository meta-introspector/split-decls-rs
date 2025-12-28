macro_rules! btrfs {
    () => {
        # [cfg (feature = "btrfs")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/btrfs.rs"] pub mod btrfs ;
    };
}

btrfs!()