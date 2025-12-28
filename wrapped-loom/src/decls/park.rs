macro_rules! park {
    () => {
        # [doc = " Mock implementation of `std::thread::park`."] # [doc = ""] # [doc = "  Blocks unless or until the current thread's token is made available."] # [doc = ""] # [doc = " A call to `park` does not guarantee that the thread will remain parked"] # [doc = " forever, and callers should be prepared for this possibility."] # [track_caller] pub fn park () { rt :: park (location ! ()) ; }
    };
}

park!()