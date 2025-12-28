macro_rules! Context {
    () => {
        # [doc = " A context."] # [derive (Clone)] pub struct Context { buffer : [u8 ; 64] , count : u64 , state : [u32 ; 4] , }
    };
}

Context!();