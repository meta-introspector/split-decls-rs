macro_rules! ImplTraitLoweringMode {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , Default)] pub (crate) enum ImplTraitLoweringMode { # [doc = " `impl Trait` gets lowered into an opaque type that doesn't unify with"] # [doc = " anything except itself. This is used in places where values flow 'out',"] # [doc = " i.e. for arguments of the function we're currently checking, and return"] # [doc = " types of functions we're calling."] Opaque , # [doc = " `impl Trait` is disallowed and will be an error."] # [default] Disallowed , }
    };
}

ImplTraitLoweringMode!();