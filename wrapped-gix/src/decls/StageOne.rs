macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! StageOne {
    () => {
        deps!();
        # [doc = " A utility to deal with the cyclic dependency between the ref store and the configuration. The ref-store needs the"] # [doc = " object hash kind, and the configuration needs the current branch name to resolve conditional includes with `onbranch`."] pub (crate) struct StageOne { pub git_dir_config : gix_config :: File < 'static > , pub buf : Vec < u8 > , pub is_bare : bool , pub lossy : bool , pub object_hash : gix_hash :: Kind , pub reflog : Option < gix_ref :: store :: WriteReflog > , pub precompose_unicode : bool , pub protect_windows : bool , }
    };
}

StageOne!();