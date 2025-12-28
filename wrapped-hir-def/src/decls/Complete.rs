macro_rules! Complete {
    () => {
        # [doc = " `#[rust_analyzer::completions(...)]` options."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum Complete { # [doc = " No `#[rust_analyzer::completions(...)]`."] Yes , # [doc = " `#[rust_analyzer::completions(ignore_flyimport)]`."] IgnoreFlyimport , # [doc = " `#[rust_analyzer::completions(ignore_flyimport_methods)]` (on a trait only)."] IgnoreFlyimportMethods , # [doc = " `#[rust_analyzer::completions(ignore_methods)]` (on a trait only)."] IgnoreMethods , }
    };
}

Complete!();