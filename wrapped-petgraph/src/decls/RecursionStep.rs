macro_rules! RecursionStep {
    () => {
        # [doc = " Small helper enum that defines the various splitup recursion steps of Tarjan's algorithm."] enum RecursionStep { BaseStep (usize) , ProcessChildStep (usize , usize) , NoBackEdgeConditionCheck (usize , usize) , RootMoreThanTwoChildrenCheck (usize) , }
    };
}

RecursionStep!()