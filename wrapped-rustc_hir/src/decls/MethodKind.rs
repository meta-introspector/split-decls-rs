macro_rules! MethodKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Debug , Eq , HashStable_Generic)] pub enum MethodKind { # [doc = " Method in a `trait Trait` block"] Trait { # [doc = " Whether a default is provided for this method"] body : bool , } , # [doc = " Method in a `impl Trait for Type` block"] TraitImpl , # [doc = " Method in a `impl Type` block"] Inherent , }
    };
}

MethodKind!();