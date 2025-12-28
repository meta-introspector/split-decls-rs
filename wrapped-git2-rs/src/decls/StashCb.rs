macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! StashCb {
    () => {
        deps!();
        # [doc = " This is a callback function you can provide to iterate over all the"] # [doc = " stashed states that will be invoked per entry."] pub type StashCb < 'a > = dyn FnMut (usize , & str , & Oid) -> bool + 'a ;
    };
}

StashCb!()