macro_rules! _export {
    () => {
        # [doc = " Implementation details for macros."] # [doc = " Do not use. Used for macros only. Not covered by semver guarantees."] # [doc (hidden)] pub mod _export { pub use crate :: string :: format ; }
    };
}

_export!();