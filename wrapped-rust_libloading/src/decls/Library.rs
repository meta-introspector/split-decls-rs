macro_rules! Library {
    () => {
        # [doc = " A loaded dynamic library."] # [cfg_attr (libloading_docs , doc (cfg (any (unix , windows))))] pub struct Library (imp :: Library) ;
    };
}

Library!();