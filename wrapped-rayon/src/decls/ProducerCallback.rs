macro_rules! deps {
    () => {
        Producer!();
    };
}

macro_rules! ProducerCallback {
    () => {
        deps!();
        # [doc = " The `ProducerCallback` trait is a kind of generic closure,"] # [doc = " [analogous to `FnOnce`][FnOnce]. See [the corresponding section in"] # [doc = " the plumbing README][r] for more details."] # [doc = ""] # [doc = " [r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md#producer-callback"] # [doc = " [FnOnce]: std::ops::FnOnce"] pub trait ProducerCallback < T > { # [doc = " The type of value returned by this callback. Analogous to"] # [doc = " [`Output` from the `FnOnce` trait][Output]."] # [doc = ""] # [doc = " [Output]: std::ops::FnOnce::Output"] type Output ; # [doc = " Invokes the callback with the given producer as argument. The"] # [doc = " key point of this trait is that this method is generic over"] # [doc = " `P`, and hence implementors must be defined for any producer."] fn callback < P > (self , producer : P) -> Self :: Output where P : Producer < Item = T > ; }
    };
}

ProducerCallback!()