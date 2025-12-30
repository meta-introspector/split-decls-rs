// Generated macro for if_strategy_present (function)
macro_rules! Depcrate_errorif_strategy_present {
() => {
// Module: crate::error
// Provides: {"if_strategy_present"}
// Dependencies: {}
# [doc = " Ensures that an explicit strategy or value is not present on `item`."] pub fn if_strategy_present (ctx : Ctx , attrs : & ParsedAttributes , item : & str) { use crate :: attr :: StratMode :: * ; match attrs . strategy { Arbitrary => { } Strategy (_) => illegal_strategy (ctx , "strategy" , item) , Value (_) => illegal_strategy (ctx , "value" , item) , Regex (_) => illegal_regex (ctx , item) , } }
};
}
