macro_rules! deps {
    () => {
        LetKind!();
    };
}

macro_rules! ExtendedTemporaryScope {
    () => {
        deps!();
        # [derive (Copy , Clone)] struct ExtendedTemporaryScope { # [doc = " The scope of extended temporaries."] scope : Option < Scope > , # [doc = " Whether this lifetime originated from a regular `let` or a `super let` initializer. In the"] # [doc = " latter case, this scope may shorten after #145838 if applied to temporaries within block"] # [doc = " tail expressions."] let_kind : LetKind , # [doc = " Whether this scope will shorten after #145838. If this is applied to a temporary value,"] # [doc = " we'll emit the `macro_extended_temporary_scopes` lint."] compat : ScopeCompatibility , }
    };
}

ExtendedTemporaryScope!();