macro_rules! Context {
    () => {
        # [derive (Debug , Copy , Clone)] struct Context { # [doc = " The scope that contains any new variables declared."] var_parent : (Option < Scope > , ScopeCompatibility) , # [doc = " Region parent of expressions, etc."] parent : Option < Scope > , }
    };
}

Context!()