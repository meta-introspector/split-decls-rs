macro_rules! prefix {
    () => {
        # [cfg (all (not (feature = "custom-prefix") , not (feature = "semver-prefix") , any (test , feature = "testing-prefix")))] macro_rules ! prefix { ($ name : expr) => { concat ! ("LIBZ_RS_SYS_TEST_" , stringify ! ($ name)) } ; }
    };
}

prefix!()