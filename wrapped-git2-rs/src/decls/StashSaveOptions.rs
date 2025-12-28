macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! StashSaveOptions {
    () => {
        deps!();
        # [doc = " Stash application options structure"] pub struct StashSaveOptions < 'a > { message : Option < CString > , flags : Option < StashFlags > , stasher : Signature < 'a > , pathspec : Vec < CString > , pathspec_ptrs : Vec < * const c_char > , raw_opts : raw :: git_stash_save_options , }
    };
}

StashSaveOptions!()