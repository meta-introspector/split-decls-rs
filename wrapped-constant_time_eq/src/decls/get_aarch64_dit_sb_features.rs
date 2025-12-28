macro_rules! deps {
    () => {
        Features!();
    };
}

macro_rules! get_aarch64_dit_sb_features {
    () => {
        deps!();
        # [doc = " Determines whether `FEAT_DIT` and `FEAT_SB` are known to be implemented."] # [cfg (all (target_feature = "dit" , target_feature = "sb"))] # [inline (always)] fn get_aarch64_dit_sb_features () -> Features { Features :: DitSb }
    };
}

get_aarch64_dit_sb_features!()