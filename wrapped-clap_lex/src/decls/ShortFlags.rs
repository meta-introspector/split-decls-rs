macro_rules! deps {
    () => {
        ParsedArg!();
    };
}

macro_rules! ShortFlags {
    () => {
        deps!();
        # [doc = " Walk through short flags within a [`ParsedArg`]"] # [derive (Clone , Debug)] pub struct ShortFlags < 's > { inner : & 's OsStr , utf8_prefix : std :: str :: CharIndices < 's > , invalid_suffix : Option < & 's OsStr > , }
    };
}

ShortFlags!()