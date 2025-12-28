macro_rules! kem_interface {
    () => {
        # [cfg (feature = "safe_api")] # [doc = " Tests for KEMs such as `mlkem`."] pub mod kem_interface ;
    };
}

kem_interface!()