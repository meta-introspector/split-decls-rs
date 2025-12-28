macro_rules! hpke_interface {
    () => {
        # [cfg (all (test , feature = "safe_api"))] # [doc = " Tests for HPKE."] pub mod hpke_interface ;
    };
}

hpke_interface!()