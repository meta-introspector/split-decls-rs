macro_rules! ParseOptions {
    () => {
        # [doc = " Options to control the parsing process."] # [derive (Clone , Copy , Debug , Default)] # [repr (C)] pub struct ParseOptions { recursion_limit : Option < NonZeroU32 > , }
    };
}

ParseOptions!();