macro_rules! macro_102 {
    () => {
        pin_project ! { # [doc = " Future for the [`StreamExt::partition()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PartitionFuture < S , P , B > { # [pin] stream : S , predicate : P , res : Option < (B , B) >, } }
    };
}

macro_102!();