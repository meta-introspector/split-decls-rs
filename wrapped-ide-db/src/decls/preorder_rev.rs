macro_rules! preorder_rev {
    () => {
        fn preorder_rev (item : & SyntaxNode) -> impl Iterator < Item = SyntaxNode > { let x = item . preorder () . filter_map (| event | match event { syntax :: WalkEvent :: Enter (node) => Some (node) , syntax :: WalkEvent :: Leave (_) => None , }) . collect_vec () ; x . into_iter () . rev () }
    };
}

preorder_rev!()