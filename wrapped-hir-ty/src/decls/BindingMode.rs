macro_rules! BindingMode {
    () => {
        # [doc = " Binding modes inferred for patterns."] # [doc = " <https://doc.rust-lang.org/reference/patterns.html#binding-modes>"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Default)] pub enum BindingMode { # [default] Move , Ref (Mutability) , }
    };
}

BindingMode!()