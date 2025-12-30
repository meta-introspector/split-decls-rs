// Generated macro for repeat_until (function)
macro_rules! Depcrate_parser_repeatrepeat_until {
() => {
// Module: crate::parser::repeat
// Provides: {"repeat_until"}
// Dependencies: {}
pub fn repeat_until < F , Input , P , E > (parser : P , end : E) -> RepeatUntil < F , P , E > where Input : Stream , F : Extend < P :: Output > + Default , P : Parser < Input > , E : Parser < Input > , { RepeatUntil { parser , end , _marker : PhantomData , } }
};
}
