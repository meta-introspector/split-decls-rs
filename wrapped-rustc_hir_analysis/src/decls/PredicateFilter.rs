macro_rules! PredicateFilter {
    () => {
        # [derive (Copy , Clone , Debug)] pub enum PredicateFilter { # [doc = " All predicates may be implied by the trait."] All , # [doc = " Only traits that reference `Self: ..` are implied by the trait."] SelfOnly , # [doc = " Only traits that reference `Self: ..` and define an associated type"] # [doc = " with the given ident are implied by the trait. This mode exists to"] # [doc = " side-step query cycles when lowering associated types."] SelfTraitThatDefines (Ident) , # [doc = " Only traits that reference `Self: ..` and their associated type bounds."] # [doc = " For example, given `Self: Tr<A: B>`, this would expand to `Self: Tr`"] # [doc = " and `<Self as Tr>::A: B`."] SelfAndAssociatedTypeBounds , # [doc = " Filter only the `[const]` bounds, which are lowered into `HostEffect` clauses."] ConstIfConst , # [doc = " Filter only the `[const]` bounds which are *also* in the supertrait position."] SelfConstIfConst , }
    };
}

PredicateFilter!()