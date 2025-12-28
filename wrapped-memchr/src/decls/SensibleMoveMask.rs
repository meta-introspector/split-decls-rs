macro_rules! SensibleMoveMask {
    () => {
        # [doc = " This is a \"sensible\" movemask implementation where each bit represents"] # [doc = " whether the most significant bit is set in each corresponding lane of a"] # [doc = " vector. This is used on x86-64 and wasm, but such a mask is more expensive"] # [doc = " to get on aarch64 so we use something a little different."] # [doc = ""] # [doc = " We call this \"sensible\" because this is what we get using native sse/avx"] # [doc = " movemask instructions. But neon has no such native equivalent."] # [derive (Clone , Copy , Debug)] pub (crate) struct SensibleMoveMask (u32) ;
    };
}

SensibleMoveMask!();