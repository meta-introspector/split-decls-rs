macro_rules! deps {
    () => {
        Change!();
    };
}

macro_rules! ChangeId {
    () => {
        deps!();
        # [doc = " A way to recognize and associate different [`Change`] instances."] # [doc = ""] # [doc = " These are unique only within one diff operation."] pub type ChangeId = u32 ;
    };
}

ChangeId!()