macro_rules! deps {
    () => {
        BindingAnnotation!();
        HygieneId!();
        BindingProblems!();
    };
}

macro_rules! Binding {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct Binding { pub name : Name , pub mode : BindingAnnotation , pub problems : Option < BindingProblems > , # [doc = " Note that this may not be the direct `SyntaxContextId` of the binding's expansion, because transparent"] # [doc = " expansions are attributed to their parent expansion (recursively)."] pub hygiene : HygieneId , }
    };
}

Binding!()