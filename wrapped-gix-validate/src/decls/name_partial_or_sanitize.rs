macro_rules! deps {
    () => {
        Mode!();
    };
}

macro_rules! name_partial_or_sanitize {
    () => {
        deps!();
        # [doc = " The infallible version of [`name_partial()`] which instead of failing, alters `path` and returns it to be a valid"] # [doc = " partial name, which would also pass [`name_partial()`]."] # [doc = ""] # [doc = " Note that an empty `path` is replaced with a `-` in order to be valid."] pub fn name_partial_or_sanitize (path : & BStr) -> BString { validate (path , Mode :: PartialSanitize) . expect ("BUG: errors cannot happen as any issue is fixed instantly") . expect ("we always rebuild the path") }
    };
}

name_partial_or_sanitize!()