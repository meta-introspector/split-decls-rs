macro_rules! OpenHow {
    () => {
        # [doc = " Wrapper around `open_how` as used in [the `openat2(2)` system"] # [doc = " call](https://man7.org/linux/man-pages/man2/openat2.2.html)."] # [derive (Default , Debug , Clone , Copy)] # [repr (transparent)] pub struct OpenHow (sys :: open_how) ;
    };
}

OpenHow!()