macro_rules! PatternError {
    () => {
        # [doc = " A pattern parsing error."] # [derive (Debug)] # [allow (missing_copy_implementations)] pub struct PatternError { # [doc = " The approximate character index of where the error occurred."] pub pos : usize , # [doc = " A message describing the error."] pub msg : & 'static str , }
    };
}

PatternError!()