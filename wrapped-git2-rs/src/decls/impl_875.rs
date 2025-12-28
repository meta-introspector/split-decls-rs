macro_rules! deps {
    () => {
        ConfigLevel!();
        Worktree!();
    };
}

macro_rules! impl_875 {
    () => {
        deps!();
        impl ConfigLevel { # [doc = " Converts a raw configuration level to a ConfigLevel"] pub fn from_raw (raw : raw :: git_config_level_t) -> ConfigLevel { match raw { raw :: GIT_CONFIG_LEVEL_PROGRAMDATA => ConfigLevel :: ProgramData , raw :: GIT_CONFIG_LEVEL_SYSTEM => ConfigLevel :: System , raw :: GIT_CONFIG_LEVEL_XDG => ConfigLevel :: XDG , raw :: GIT_CONFIG_LEVEL_GLOBAL => ConfigLevel :: Global , raw :: GIT_CONFIG_LEVEL_LOCAL => ConfigLevel :: Local , raw :: GIT_CONFIG_LEVEL_WORKTREE => ConfigLevel :: Worktree , raw :: GIT_CONFIG_LEVEL_APP => ConfigLevel :: App , raw :: GIT_CONFIG_HIGHEST_LEVEL => ConfigLevel :: Highest , n => panic ! ("unknown config level: {}" , n) , } } }
    };
}

impl_875!()