macro_rules! BindingAnnotation {
    () => {
        # [doc = " Explicit binding annotations given in the HIR for a binding. Note"] # [doc = " that this is not the final binding *mode* that we infer after type"] # [doc = " inference."] # [derive (Clone , PartialEq , Eq , Debug , Copy)] pub enum BindingAnnotation { # [doc = " No binding annotation given: this means that the final binding mode"] # [doc = " will depend on whether we have skipped through a `&` reference"] # [doc = " when matching. For example, the `x` in `Some(x)` will have binding"] # [doc = " mode `None`; if you do `let Some(x) = &Some(22)`, it will"] # [doc = " ultimately be inferred to be by-reference."] Unannotated , # [doc = " Annotated with `mut x` -- could be either ref or not, similar to `None`."] Mutable , # [doc = " Annotated as `ref`, like `ref x`"] Ref , # [doc = " Annotated as `ref mut x`."] RefMut , }
    };
}

BindingAnnotation!()