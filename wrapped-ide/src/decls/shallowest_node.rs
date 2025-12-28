macro_rules! shallowest_node {
    () => {
        # [doc = " Find the shallowest node with same range, which allows us to traverse siblings."] fn shallowest_node (node : & SyntaxNode) -> SyntaxNode { node . ancestors () . take_while (| n | n . text_range () == node . text_range ()) . last () . unwrap () }
    };
}

shallowest_node!();