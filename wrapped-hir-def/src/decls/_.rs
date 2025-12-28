macro_rules! deps {
    () => {
        SmallModItem!();
    };
}

macro_rules! _ {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] const _ : [() ; std :: mem :: size_of :: < SmallModItem > ()] = [() ; std :: mem :: size_of :: < [usize ; 3] > ()] ;
    };
}

_!();