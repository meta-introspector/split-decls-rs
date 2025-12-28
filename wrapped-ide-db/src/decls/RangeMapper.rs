macro_rules! RangeMapper {
    () => {
        # [derive (Default)] pub struct RangeMapper { buf : String , ranges : Vec < (TextRange , Option < TextRange >) > , }
    };
}

RangeMapper!();