macro_rules! SMALL {
    () => {
        # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] pub (crate) const SMALL : usize = 30 ;
    };
}

SMALL!();