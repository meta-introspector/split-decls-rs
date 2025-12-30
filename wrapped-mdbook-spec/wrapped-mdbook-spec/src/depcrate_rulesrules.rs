// Generated macro for Rules (struct)
macro_rules! Depcrate_rulesRules {
() => {
// Module: crate::rules
// Provides: {"Rules"}
// Dependencies: {}
# [doc = " The set of rules defined in the reference."] # [derive (Default)] pub struct Rules { # [doc = " A mapping from a rule identifier to a tuple of `(source_path, path)`."] # [doc = ""] # [doc = " `source_path` is the path to the markdown source file relative to the"] # [doc = " `SUMMARY.md`."] # [doc = ""] # [doc = " `path` is the same as `source_path`, except filenames like `README.md`"] # [doc = " are translated to `index.md`. Which to use depends on if you are"] # [doc = " trying to access the source files (`source_path`), or creating links"] # [doc = " in the output (`path`)."] pub def_paths : BTreeMap < String , (PathBuf , PathBuf) > , # [doc = " Set of rule name prefixes that have more specific rules within."] # [doc = ""] # [doc = " For example, `asm.ts-args` is an interior prefix of `asm.ts-args.syntax`."] pub interior_prefixes : HashSet < String > , }
};
}
