macro_rules! deps {
    () => {
        GatedCfg!();
    };
}

macro_rules! find_gated_cfg {
    () => {
        deps!();
        # [doc = " Find a gated cfg determined by the `pred`icate which is given the cfg's name."] pub fn find_gated_cfg (pred : impl Fn (Symbol) -> bool) -> Option < & 'static GatedCfg > { GATED_CFGS . iter () . find (| (cfg_sym , ..) | pred (* cfg_sym)) }
    };
}

find_gated_cfg!()