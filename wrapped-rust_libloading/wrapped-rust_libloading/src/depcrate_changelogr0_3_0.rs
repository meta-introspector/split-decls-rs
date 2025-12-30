// Generated macro for r0_3_0 (module)
macro_rules! Depcrate_changelogr0_3_0 {
() => {
// Module: crate::changelog
// Provides: {"r0_3_0"}
// Dependencies: {}
# [doc = " Release 0.3.0 (2016-07-27)"] # [doc = ""] # [doc = " * Greatly improved documentation, especially around platform-specific behaviours;"] # [doc = " * Improved test suite by building our own library to test against;"] # [doc = " * All `Library`-ies now implement `Send`."] # [doc = " * Added `impl From<os::platform::Library> for Library` and `impl From<Library> for"] # [doc = "   os::platform::Library` allowing wrapping and extracting the platform-specific library handle;"] # [doc = " * Added methods to wrap (`Symbol::from_raw`) and unwrap (`Symbol::into_raw`) the safe `Symbol`"] # [doc = "   wrapper into unsafe `os::platform::Symbol`."] # [doc = ""] # [doc = " The last two additions focus on not restricting potential usecases of this library, allowing"] # [doc = " users of the library to circumvent safety checks if need be."] # [doc = ""] # [doc = " ## Breaking Changes"] # [doc = ""] # [doc = " `Library::new` defaults to `RTLD_NOW` instead of `RTLD_LAZY` on UNIX for more consistent"] # [doc = " cross-platform behaviour. If a library loaded with `Library::new` had any linking errors, but"] # [doc = " unresolved references weren’t forced to be resolved, the library would’ve “just worked”,"] # [doc = " whereas now the call to `Library::new` will return an error signifying presence of such error."] # [doc = ""] # [doc = " ## os::platform"] # [doc = " * Added `os::unix::Library::open` which allows specifying arbitrary flags (e.g. `RTLD_LAZY`);"] # [doc = " * Added `os::windows::Library::get_ordinal` which allows finding a function or variable by its"] # [doc = "   ordinal number;"] pub mod r0_3_0 { }
};
}
