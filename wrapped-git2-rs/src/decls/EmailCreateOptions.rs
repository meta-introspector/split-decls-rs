macro_rules! deps {
    () => {
        DiffFindOptions!();
        DiffOptions!();
    };
}

macro_rules! EmailCreateOptions {
    () => {
        deps!();
        # [doc = " Options for controlling the formatting of the generated e-mail."] pub struct EmailCreateOptions { diff_options : DiffOptions , diff_find_options : DiffFindOptions , subject_prefix : Option < CString > , raw : raw :: git_email_create_options , }
    };
}

EmailCreateOptions!()