macro_rules! AlgoResult {
    () => {
        # [doc = " Return value of [`with_dynamic_goal`]."] pub struct AlgoResult < N , K > { # [doc = " A [`struct@hashbrown::HashMap`] that maps `NodeId` to path cost."] pub scores : HashMap < N , K > , # [doc = " The goal node that terminated the search, if any was found."] pub goal_node : Option < N > , }
    };
}

AlgoResult!();