macro_rules! deps {
    () => {
        Size!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl Step for Size { # [inline] fn steps_between (start : & Self , end : & Self) -> (usize , Option < usize >) { u64 :: steps_between (& start . bytes () , & end . bytes ()) } # [inline] fn forward_checked (start : Self , count : usize) -> Option < Self > { u64 :: forward_checked (start . bytes () , count) . map (Self :: from_bytes) } # [inline] fn forward (start : Self , count : usize) -> Self { Self :: from_bytes (u64 :: forward (start . bytes () , count)) } # [inline] unsafe fn forward_unchecked (start : Self , count : usize) -> Self { Self :: from_bytes (unsafe { u64 :: forward_unchecked (start . bytes () , count) }) } # [inline] fn backward_checked (start : Self , count : usize) -> Option < Self > { u64 :: backward_checked (start . bytes () , count) . map (Self :: from_bytes) } # [inline] fn backward (start : Self , count : usize) -> Self { Self :: from_bytes (u64 :: backward (start . bytes () , count)) } # [inline] unsafe fn backward_unchecked (start : Self , count : usize) -> Self { Self :: from_bytes (unsafe { u64 :: backward_unchecked (start . bytes () , count) }) } }
    };
}

impl_35!()