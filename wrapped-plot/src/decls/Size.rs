macro_rules! deps {
    () => {
        Figure!();
    };
}

macro_rules! Size {
    () => {
        deps!();
        # [doc = " Figure size"] # [derive (Clone , Copy)] pub struct Size (pub usize , pub usize) ;
    };
}

Size!();