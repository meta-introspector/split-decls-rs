macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'a > Options < 'a > { # [doc = " Provide options to never follow include directives at all."] pub fn no_follow () -> Self { Options { max_depth : 0 , err_on_max_depth_exceeded : false , err_on_interpolation_failure : false , err_on_missing_config_path : false , interpolate : Default :: default () , conditional : Default :: default () , } } # [doc = " Provide options to follow includes like git does, provided the required `conditional` and `interpolate` contexts"] # [doc = " to support `gitdir` and `onbranch` based `includeIf` directives as well as standard `include.path` resolution."] # [doc = " Note that the follow-mode is `git`-style, following at most 10 indirections while"] # [doc = " producing an error if the depth is exceeded."] pub fn follow (interpolate : interpolate :: Context < 'a > , conditional : conditional :: Context < 'a >) -> Self { Options { max_depth : 10 , err_on_max_depth_exceeded : true , err_on_interpolation_failure : false , err_on_missing_config_path : true , interpolate , conditional , } } # [doc = " For use with `follow` type options, cause failure if an include path couldn't be interpolated or the depth limit is exceeded."] pub fn strict (mut self) -> Self { self . err_on_interpolation_failure = true ; self . err_on_max_depth_exceeded = true ; self . err_on_missing_config_path = true ; self } # [doc = " Like [`follow`][Options::follow()], but without information to resolve `includeIf` directories as well as default"] # [doc = " configuration to allow resolving `~username/` path. `home_dir` is required to resolve `~/` paths if set."] # [doc = " Note that `%(prefix)` paths cannot be interpolated with this configuration, use [`follow()`][Options::follow()]"] # [doc = " instead for complete control."] pub fn follow_without_conditional (home_dir : Option < & 'a std :: path :: Path >) -> Self { Options { max_depth : 10 , err_on_max_depth_exceeded : true , err_on_interpolation_failure : false , err_on_missing_config_path : true , interpolate : interpolate :: Context { git_install_dir : None , home_dir , home_for_user : Some (interpolate :: home_for_user) , } , conditional : Default :: default () , } } # [doc = " Set the context used for interpolation when interpolating paths to include as well as the paths"] # [doc = " in `gitdir` conditional includes."] pub fn interpolate_with (mut self , context : interpolate :: Context < 'a >) -> Self { self . interpolate = context ; self } }
    };
}

impl_64!()