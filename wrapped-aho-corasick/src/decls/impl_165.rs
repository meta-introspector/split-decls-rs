macro_rules! deps {
    () => {
        Vector!();
        Match!();
        Teddy!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl Teddy < 8 > { # [doc = " Runs the verification routine for \"slim\" Teddy."] # [doc = ""] # [doc = " The candidate given should be a collection of 8-bit bitsets (one bitset"] # [doc = " per lane), where the ith bit is set in the jth lane if and only if the"] # [doc = " byte occurring at `at + j` in `cur` is in the bucket `i`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that this is okay to call in the current target for"] # [doc = " the current CPU."] # [doc = ""] # [doc = " The given pointers must be valid to read from."] # [inline (always)] unsafe fn verify < V : Vector > (& self , mut cur : * const u8 , end : * const u8 , candidate : V ,) -> Option < Match > { debug_assert ! (! candidate . is_zero ()) ; candidate . for_each_64bit_lane (# [inline (always)] | _ , chunk | { let result = self . verify64 (cur , end , chunk) ; cur = cur . add (8) ; result } ,) } }
    };
}

impl_165!()