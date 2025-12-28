macro_rules! VisibilityExplicitness {
    () => {
        # [doc = " Whether the item was imported through an explicit `pub(crate) use` or just a `use` without"] # [doc = " visibility."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum VisibilityExplicitness { Explicit , Implicit , }
    };
}

VisibilityExplicitness!()