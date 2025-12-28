macro_rules! StatusOptions {
    () => {
        # [doc = " Options that can be provided to `repo.statuses()` to control how the status"] # [doc = " information is gathered."] pub struct StatusOptions { raw : raw :: git_status_options , pathspec : Vec < CString > , ptrs : Vec < * const c_char > , }
    };
}

StatusOptions!();