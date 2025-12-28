macro_rules! deps {
    () => {
        InferCtxt!();
    };
}

macro_rules! FullTypeResolver {
    () => {
        deps!();
        struct FullTypeResolver < 'a , 'tcx > { infcx : & 'a InferCtxt < 'tcx > , }
    };
}

FullTypeResolver!();