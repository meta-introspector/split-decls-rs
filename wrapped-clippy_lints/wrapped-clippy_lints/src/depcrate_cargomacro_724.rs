// Generated macro for macro_724 (macro)
macro_rules! Depcrate_cargomacro_724 {
() => {
// Module: crate::cargo
// Provides: {"macro_724"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for lint groups with the same priority as lints in the `Cargo.toml`"] # [doc = " [`[lints]` table](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section)."] # [doc = ""] # [doc = " This lint will be removed once [cargo#12918](https://github.com/rust-lang/cargo/issues/12918)"] # [doc = " is resolved."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The order of lints in the `[lints]` is ignored, to have a lint override a group the"] # [doc = " `priority` field needs to be used, otherwise the sort order is undefined."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Does not check lints inherited using `lints.workspace = true`"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```toml"] # [doc = " # Passed as `--allow=clippy::similar_names --warn=clippy::pedantic`"] # [doc = " # which results in `similar_names` being `warn`"] # [doc = " [lints.clippy]"] # [doc = " pedantic = \"warn\""] # [doc = " similar_names = \"allow\""] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```toml"] # [doc = " # Passed as `--warn=clippy::pedantic --allow=clippy::similar_names`"] # [doc = " # which results in `similar_names` being `allow`"] # [doc = " [lints.clippy]"] # [doc = " pedantic = { level = \"warn\", priority = -1 }"] # [doc = " similar_names = \"allow\""] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub LINT_GROUPS_PRIORITY , correctness , "a lint group in `Cargo.toml` at the same priority as a lint" }
};
}
