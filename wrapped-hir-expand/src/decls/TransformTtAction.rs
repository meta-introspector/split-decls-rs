macro_rules! TransformTtAction {
    () => {
        # [derive (Debug)] enum TransformTtAction < 'a > { Keep , ReplaceWith (tt :: TokenTreesView < 'a >) , }
    };
}

TransformTtAction!();