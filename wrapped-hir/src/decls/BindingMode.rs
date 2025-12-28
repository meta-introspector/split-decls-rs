macro_rules! BindingMode {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum BindingMode { Move , Ref (Mutability) , }
    };
}

BindingMode!()