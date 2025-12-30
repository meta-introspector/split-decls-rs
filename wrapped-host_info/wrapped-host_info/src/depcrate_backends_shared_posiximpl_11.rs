// Generated macro for impl_11 (impl)
macro_rules! Depcrate_backends_shared_posiximpl_11 {
() => {
// Module: crate::backends::shared::posix
// Provides: {"impl_11"}
// Dependencies: {}
impl LocaleCategory { # [inline] fn to_env_var_name (self) -> & 'static str { match self { LocaleCategory :: Character => "LC_CTYPE" , LocaleCategory :: Number => "LC_NUMERIC" , LocaleCategory :: Time => "LC_TIME" , LocaleCategory :: Collate => "LC_COLLATE" , LocaleCategory :: Monetary => "LC_MONETARY" , LocaleCategory :: Messages => "LC_MESSAGES" , LocaleCategory :: Paper => "LC_PAPER" , LocaleCategory :: Name => "LC_NAME" , LocaleCategory :: Address => "LC_ADDRESS" , LocaleCategory :: Telephone => "LC_TELEPHONE" , LocaleCategory :: Measurement => "LC_MEASUREMENT" , LocaleCategory :: Identification => "LC_IDENTIFICATION" , LocaleCategory :: All => "LC_ALL" , } } }
};
}
