macro_rules! if_ether {
    () => {
        # [cfg (feature = "if_ether")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/if_ether.rs"] pub mod if_ether ;
    };
}

if_ether!()