macro_rules! CaptureNames {
    () => {
        # [doc = " An iterator over the names of all capture groups in a regex."] # [doc = ""] # [doc = " This iterator yields values of type `Option<&str>` in order of the opening"] # [doc = " capture group parenthesis in the regex pattern. `None` is yielded for"] # [doc = " groups with no name. The first element always corresponds to the implicit"] # [doc = " and unnamed group for the overall match."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression."] # [doc = ""] # [doc = " This iterator is created by [`Regex::capture_names`]."] # [derive (Clone , Debug)] pub struct CaptureNames < 'r > (captures :: GroupInfoPatternNames < 'r >) ;
    };
}

CaptureNames!();