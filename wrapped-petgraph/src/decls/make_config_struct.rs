macro_rules! deps {
    () => {
        Config!();
        RankDir!();
    };
}

macro_rules! make_config_struct {
    () => {
        deps!();
        macro_rules ! make_config_struct { ($ ($ variant : ident ,) *) => { # [allow (non_snake_case)] # [derive (Default)] struct Configs { $ ($ variant : bool ,) * RankDir : Option < RankDir >, } impl Configs { # [inline] fn extract (configs : & [Config]) -> Self { let mut conf = Self :: default () ; for c in configs { match c { $ (Config ::$ variant => conf .$ variant = true ,) * Config :: RankDir (dir) => conf . RankDir = Some (* dir) , } } conf } } } }
    };
}

make_config_struct!()