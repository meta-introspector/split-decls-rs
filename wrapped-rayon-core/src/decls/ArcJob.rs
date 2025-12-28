macro_rules! deps {
    () => {
        HeapJob!();
    };
}

macro_rules! ArcJob {
    () => {
        deps!();
        # [doc = " Represents a job stored in an `Arc` -- like `HeapJob`, but may"] # [doc = " be turned into multiple `JobRef`s and called multiple times."] pub (super) struct ArcJob < BODY > where BODY : Fn () + Send + Sync , { job : BODY , }
    };
}

ArcJob!()