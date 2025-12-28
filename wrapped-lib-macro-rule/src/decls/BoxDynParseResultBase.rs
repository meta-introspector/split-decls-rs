macro_rules! deps {
    () => {
        ParseResultBase!();
    };
}

macro_rules! BoxDynParseResultBase {
    () => {
        deps!();
        # [macro_export] macro_rules ! BoxDynParseResultBase { () => { Box < dyn $ crate :: parse_result :: ParseResultBase < () >> } ; }
    };
}

BoxDynParseResultBase!()