macro_rules! if_arp {
    () => {
        # [cfg (feature = "if_arp")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/if_arp.rs"] pub mod if_arp ;
    };
}

if_arp!()