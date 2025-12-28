macro_rules! Patchwork {
    () => {
        # [derive (Debug)] struct Patchwork { text : String , indels : Vec < (Range < usize > , usize) > , }
    };
}

Patchwork!()