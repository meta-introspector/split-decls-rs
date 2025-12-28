macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_657 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < St , Item > Sink < Item > for NestedTryStreamIntoEitherTryStream < St > where St : TryStream + Sink < Item > , St :: Ok : TryStream + Unpin , < St :: Ok as TryStream > :: Error : From < < St as TryStream > :: Error > , { type Error = < St as Sink < Item > > :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_657!();