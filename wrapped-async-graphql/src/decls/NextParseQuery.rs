macro_rules! deps {
    () => {
        Extension!();
        ParseFut!();
    };
}

macro_rules! NextParseQuery {
    () => {
        deps!();
        # [doc = " The remainder of a extension chain for parse query."] pub struct NextParseQuery < 'a > { chain : & 'a [Arc < dyn Extension >] , parse_query_fut : ParseFut < 'a > , }
    };
}

NextParseQuery!();