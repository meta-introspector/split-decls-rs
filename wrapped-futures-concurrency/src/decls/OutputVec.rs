macro_rules! OutputVec {
    () => {
        # [doc = " A contiguous vector of uninitialized data."] pub (crate) struct OutputVec < T > { data : Vec < T > , capacity : usize , }
    };
}

OutputVec!()