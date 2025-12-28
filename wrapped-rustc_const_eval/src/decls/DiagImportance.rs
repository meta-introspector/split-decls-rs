macro_rules! DiagImportance {
    () => {
        # [derive (Clone , Copy)] pub enum DiagImportance { # [doc = " An operation that must be removed for const-checking to pass."] Primary , # [doc = " An operation that causes const-checking to fail, but is usually a side-effect of a `Primary` operation elsewhere."] Secondary , }
    };
}

DiagImportance!();