macro_rules! deps {
    () => {
        Error!();
        Note!();
        Iter!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl Iter < '_ , '_ > { # [doc = " Automatically peel references before yielding them during iteration."] # [doc = ""] # [doc = " This has the same effect as using `iter.map(|r| {r.peel_to_id(); r})`."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " Doing this is necessary as the packed-refs buffer is already held by the iterator, disallowing the consumer of the iterator"] # [doc = " to peel the returned references themselves."] pub fn peeled (mut self) -> Result < Self , gix_ref :: packed :: buffer :: open :: Error > { self . peel_with_packed = self . repo . refs . cached_packed_buffer () ? ; self . peel = true ; Ok (self) } }
    };
}

impl_258!()