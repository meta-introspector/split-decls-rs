macro_rules! deps {
    () => {
        FatVector!();
        Teddy!();
        Match!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl Teddy < 16 > { # [doc = " Runs the verification routine for \"fat\" Teddy."] # [doc = ""] # [doc = " The candidate given should be a collection of 8-bit bitsets (one bitset"] # [doc = " per lane), where the ith bit is set in the jth lane if and only if the"] # [doc = " byte occurring at `at + (j < 16 ? j : j - 16)` in `cur` is in the"] # [doc = " bucket `j < 16 ? i : i + 8`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that this is okay to call in the current target for"] # [doc = " the current CPU."] # [doc = ""] # [doc = " The given pointers must be valid to read from."] # [inline (always)] unsafe fn verify < V : FatVector > (& self , mut cur : * const u8 , end : * const u8 , candidate : V ,) -> Option < Match > { debug_assert ! (! candidate . is_zero ()) ; let swapped = candidate . swap_halves () ; let r1 = candidate . interleave_low_8bit_lanes (swapped) ; let r2 = candidate . interleave_high_8bit_lanes (swapped) ; r1 . for_each_low_64bit_lane (r2 , # [inline (always)] | _ , chunk | { let result = self . verify64 (cur , end , chunk) ; cur = cur . add (4) ; result } ,) } }
    };
}

impl_166!();