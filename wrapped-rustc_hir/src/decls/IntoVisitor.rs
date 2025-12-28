macro_rules! deps {
    () => {
        Visitor!();
    };
}

macro_rules! IntoVisitor {
    () => {
        deps!();
        pub trait IntoVisitor < 'hir > { type Visitor : Visitor < 'hir > ; fn into_visitor (& self) -> Self :: Visitor ; }
    };
}

IntoVisitor!()