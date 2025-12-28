macro_rules! deps {
    () => {
        PacketLineRef!();
        Error!();
    };
}

macro_rules! ExhaustiveOutcome {
    () => {
        deps!();
        # [cfg (any (feature = "blocking-io" , feature = "async-io"))] pub (crate) type ExhaustiveOutcome < 'a > = (bool , Option < PacketLineRef < 'static > > , Option < std :: io :: Result < Result < PacketLineRef < 'a > , crate :: decode :: Error > > > ,) ;
    };
}

ExhaustiveOutcome!()