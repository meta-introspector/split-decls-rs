macro_rules! deps {
    () => {
        TypeTrace!();
    };
}

macro_rules! ToTrace {
    () => {
        deps!();
        pub trait ToTrace < 'tcx > : Relate < TyCtxt < 'tcx > > + Copy { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > ; }
    };
}

ToTrace!();