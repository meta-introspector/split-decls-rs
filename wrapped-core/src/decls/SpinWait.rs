macro_rules! SpinWait {
    () => {
        # [doc = " A counter used to perform exponential backoff in spin loops."] # [derive (Default)] pub struct SpinWait { counter : u32 , }
    };
}

SpinWait!()