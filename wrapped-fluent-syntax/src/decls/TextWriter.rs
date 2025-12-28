macro_rules! TextWriter {
    () => {
        # [derive (Clone , Debug , Default)] struct TextWriter { buffer : String , indent_level : usize , }
    };
}

TextWriter!()