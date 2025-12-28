macro_rules! if_packet {
    () => {
        # [cfg (feature = "if_packet")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/if_packet.rs"] pub mod if_packet ;
    };
}

if_packet!()