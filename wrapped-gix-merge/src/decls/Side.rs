macro_rules! Side {
    () => {
        # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum Side { Current , Other , # [doc = " A special marker that is just used to be able to mix-in hunks that only point to the ancestor."] # [doc = " Only `before` matters then."] Ancestor , }
    };
}

Side!()