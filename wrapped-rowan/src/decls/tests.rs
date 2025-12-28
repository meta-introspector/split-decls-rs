macro_rules! deps {
    () => {
        SyntaxNode!();
        SyntaxKind!();
        SyntaxNodePtr!();
        Language!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use crate :: { GreenNodeBuilder , Language , SyntaxKind , SyntaxNode } ; use super :: SyntaxNodePtr ; # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] struct TestLanguage ; impl Language for TestLanguage { type Kind = SyntaxKind ; fn kind_from_raw (raw : SyntaxKind) -> Self :: Kind { raw } fn kind_to_raw (kind : Self :: Kind) -> SyntaxKind { kind } } fn build_immut_tree () -> SyntaxNode < TestLanguage > { let mut builder = GreenNodeBuilder :: new () ; builder . start_node (SyntaxKind (0)) ; builder . finish_node () ; SyntaxNode :: < TestLanguage > :: new_root (builder . finish ()) } # [test] # [should_panic = "tree is mutable"] fn ensure_mut_panic_on_create () { let tree = build_immut_tree () . clone_for_update () ; SyntaxNodePtr :: new (& tree) ; } # [test] # [should_panic = "tree is mutable"] fn ensure_mut_panic_on_deref () { let tree = build_immut_tree () ; let tree_mut = tree . clone_for_update () ; let syn_ptr = SyntaxNodePtr :: new (& tree) ; syn_ptr . to_node (& tree_mut) ; } }
    };
}

tests!();