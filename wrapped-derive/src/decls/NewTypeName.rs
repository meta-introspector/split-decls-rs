macro_rules! NewTypeName {
    () => {
        # [derive (Debug)] pub enum NewTypeName { New (String) , Rust , Original , }
    };
}

NewTypeName!();