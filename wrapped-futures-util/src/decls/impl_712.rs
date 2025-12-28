macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_712 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Item , E > Sink < Item > for TryBufferUnordered < S > where S : TryStream + Sink < Item , Error = E > , S :: Ok : TryFuture < Error = E > , { type Error = E ; delegate_sink ! (stream , Item) ; }
    };
}

impl_712!();