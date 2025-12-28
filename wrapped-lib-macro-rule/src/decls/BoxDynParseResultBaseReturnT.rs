macro_rules! deps {
    () => {
        ParseResultBase!();
    };
}

macro_rules! BoxDynParseResultBaseReturnT {
    () => {
        deps!();
        # [macro_export] macro_rules ! BoxDynParseResultBaseReturnT { () => { Box < dyn $ crate :: parse_result :: ParseResultBase < () >> } ; }
    };
}

BoxDynParseResultBaseReturnT!();