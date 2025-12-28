macro_rules! RcIter {
    () => {
        # [doc = " A wrapper for `Rc<RefCell<I>>`, that implements the `Iterator` trait."] # [derive (Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct RcIter < I > { # [doc = " The boxed iterator."] pub rciter : Rc < RefCell < I > > , }
    };
}

RcIter!();