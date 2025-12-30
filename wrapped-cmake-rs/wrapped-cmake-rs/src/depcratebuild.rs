// Generated macro for build (function)
macro_rules! Depcratebuild {
() => {
// Module: crate
// Provides: {"build"}
// Dependencies: {}
# [doc = " Builds the native library rooted at `path` with the default cmake options."] # [doc = " This will return the directory in which the library was installed."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use cmake;"] # [doc = ""] # [doc = " // Builds the project in the directory located in `libfoo`, installing it"] # [doc = " // into $OUT_DIR"] # [doc = " let dst = cmake::build(\"libfoo\");"] # [doc = ""] # [doc = " println!(\"cargo:rustc-link-search=native={}\", dst.display());"] # [doc = " println!(\"cargo:rustc-link-lib=static=foo\");"] # [doc = " ```"] # [doc = ""] pub fn build < P : AsRef < Path > > (path : P) -> PathBuf { Config :: new (path . as_ref ()) . build () }
};
}
