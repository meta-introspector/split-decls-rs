macro_rules! nightly {
    () => {
        # [cfg (error_generic_member_access)] mod nightly ;
    };
}

nightly!();