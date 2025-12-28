macro_rules! deps {
    () => {
        DiffFormatEmailOptions!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        impl DiffFormatEmailOptions { # [doc = " Creates a new set of email options,"] # [doc = " initialized to the default values"] pub fn new () -> Self { let mut opts = DiffFormatEmailOptions { raw : unsafe { mem :: zeroed () } , } ; assert_eq ! (unsafe { raw :: git_diff_format_email_options_init (& mut opts . raw , 1) } , 0) ; opts } fn flag (& mut self , opt : u32 , val : bool) -> & mut Self { if val { self . raw . flags |= opt ; } else { self . raw . flags &= ! opt ; } self } # [doc = " Exclude `[PATCH]` from the subject header"] pub fn exclude_subject_patch_header (& mut self , should_exclude : bool) -> & mut Self { self . flag (raw :: GIT_DIFF_FORMAT_EMAIL_EXCLUDE_SUBJECT_PATCH_MARKER , should_exclude ,) } }
    };
}

impl_362!();