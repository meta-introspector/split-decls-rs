macro_rules! deps {
    () => {
        TreeWalkMode!();
    };
}

macro_rules! impl_817 {
    () => {
        deps!();
        impl Into < raw :: git_treewalk_mode > for TreeWalkMode { # [cfg (target_env = "msvc")] fn into (self) -> raw :: git_treewalk_mode { self as i32 } # [cfg (not (target_env = "msvc"))] fn into (self) -> raw :: git_treewalk_mode { self as u32 } }
    };
}

impl_817!();