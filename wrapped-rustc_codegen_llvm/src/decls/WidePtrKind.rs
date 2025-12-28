macro_rules! WidePtrKind {
    () => {
        # [derive (Debug , PartialEq , Eq)] pub (crate) enum WidePtrKind { Slice , Dyn , }
    };
}

WidePtrKind!();