macro_rules! Version {
    () => {
        # [doc = " Version information about libgit2 and the capabilities it supports."] pub struct Version { major : c_int , minor : c_int , rev : c_int , features : c_int , }
    };
}

Version!()