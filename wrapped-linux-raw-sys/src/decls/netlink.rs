macro_rules! netlink {
    () => {
        # [cfg (feature = "netlink")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/netlink.rs"] pub mod netlink ;
    };
}

netlink!()