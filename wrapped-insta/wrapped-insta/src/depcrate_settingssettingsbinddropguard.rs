// Generated macro for SettingsBindDropGuard (struct)
macro_rules! Depcrate_settingsSettingsBindDropGuard {
() => {
// Module: crate::settings
// Provides: {"SettingsBindDropGuard"}
// Dependencies: {}
# [doc = " Returned from [`Settings::bind_to_scope`]"] # [doc = ""] # [doc = " This type is not shareable between threads:"] # [doc = ""] # [doc = " ```compile_fail E0277"] # [doc = " let mut settings = insta::Settings::clone_current();"] # [doc = " settings.set_snapshot_suffix(\"test drop guard\");"] # [doc = " let guard = settings.bind_to_scope();"] # [doc = ""] # [doc = " std::thread::spawn(move || { let guard = guard; }); // doesn't compile"] # [doc = " ```"] # [doc = ""] # [doc = " This is to ensure tests under async runtimes like `tokio` don't show unexpected results"] # [must_use = "The guard is immediately dropped so binding has no effect. Use `let _guard = ...` to bind it."] pub struct SettingsBindDropGuard (Option < Arc < ActualSettings > > , # [doc = " A ZST that is not [`Send`] but is [`Sync`]"] # [doc = ""] # [doc = " This is necessary due to the lack of stable [negative impls](https://github.com/rust-lang/rust/issues/68318)."] # [doc = ""] # [doc = " Required as [`SettingsBindDropGuard`] modifies a thread local variable which would end up"] # [doc = " with unexpected results if sent to a different thread."] std :: marker :: PhantomData < std :: sync :: MutexGuard < 'static , () > > ,) ;
};
}
