macro_rules! deps {
    () => {
        FnKind!();
    };
}

macro_rules! InheritanceKind {
    () => {
        deps!();
        # [doc = " Given the current context(caller and callee `FnKind`), it specifies"] # [doc = " the policy of predicates and generic parameters inheritance."] # [derive (Clone , Copy , Debug , PartialEq)] enum InheritanceKind { # [doc = " Copying all predicates and parameters, including those of the parent"] # [doc = " container."] # [doc = ""] # [doc = " Boolean value defines whether the `Self` parameter or `Self: Trait`"] # [doc = " predicate are copied. It's always equal to `false` except when"] # [doc = " delegating from a free function to a trait method."] # [doc = ""] # [doc = " FIXME(fn_delegation): This often leads to type inference"] # [doc = " errors. Support providing generic arguments or restrict use sites."] WithParent (bool) , # [doc = " The trait implementation should be compatible with the original trait."] # [doc = " Therefore, for trait implementations only the method's own parameters"] # [doc = " and predicates are copied."] Own , }
    };
}

InheritanceKind!();