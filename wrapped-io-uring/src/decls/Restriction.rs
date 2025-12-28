macro_rules! Restriction {
    () => {
        # [doc = " An allowed feature of io_uring. You can set the allowed features with"] # [doc = " [`register_restrictions`](crate::Submitter::register_restrictions)."] # [repr (transparent)] pub struct Restriction (sys :: io_uring_restriction) ;
    };
}

Restriction!();