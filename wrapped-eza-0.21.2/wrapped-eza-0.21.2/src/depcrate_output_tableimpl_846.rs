// Generated macro for impl_846 (impl)
macro_rules! Depcrate_output_tableimpl_846 {
() => {
// Module: crate::output::table
// Provides: {"impl_846"}
// Dependencies: {}
impl Environment { # [cfg (unix)] pub fn lock_users (& self) -> MutexGuard < '_ , UsersCache > { self . users . lock () . unwrap () } fn load_all () -> Self { let time_offset = * Local :: now () . offset () ; let numeric = locale :: Numeric :: load_user_locale () . unwrap_or_else (| _ | locale :: Numeric :: english ()) ; # [cfg (unix)] let users = Mutex :: new (UsersCache :: new ()) ; Self { time_offset , numeric , # [cfg (unix)] users , } } }
};
}
