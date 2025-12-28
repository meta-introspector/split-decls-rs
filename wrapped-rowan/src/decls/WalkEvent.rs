macro_rules! WalkEvent {
    () => {
        # [doc = " `WalkEvent` describes tree walking process."] # [derive (Debug , Copy , Clone)] pub enum WalkEvent < T > { # [doc = " Fired before traversing the node."] Enter (T) , # [doc = " Fired after the node is traversed."] Leave (T) , }
    };
}

WalkEvent!();