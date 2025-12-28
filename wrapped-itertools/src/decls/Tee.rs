macro_rules! deps {
    () => {
        TeeBuffer!();
    };
}

macro_rules! Tee {
    () => {
        deps!();
        # [doc = " One half of an iterator pair where both return the same elements."] # [doc = ""] # [doc = " See [`.tee()`](crate::Itertools::tee) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug)] pub struct Tee < I > where I : Iterator , { rcbuffer : Rc < RefCell < TeeBuffer < I :: Item , I > > > , id : bool , }
    };
}

Tee!();