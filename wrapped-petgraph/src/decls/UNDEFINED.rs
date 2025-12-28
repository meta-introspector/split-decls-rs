macro_rules! UNDEFINED {
    () => {
        # [doc = " The undefined dominator sentinel, for when we have not yet discovered a"] # [doc = " node's dominator."] const UNDEFINED : usize = usize :: MAX ;
    };
}

UNDEFINED!()