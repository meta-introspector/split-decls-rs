macro_rules! UnsafeSource {
    () => {
        # [derive (Copy , Clone , PartialEq , Debug , HashStable_Generic)] pub enum UnsafeSource { CompilerGenerated , UserProvided , }
    };
}

UnsafeSource!()