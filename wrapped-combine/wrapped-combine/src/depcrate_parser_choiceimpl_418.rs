// Generated macro for impl_418 (impl)
macro_rules! Depcrate_parser_choiceimpl_418 {
() => {
// Module: crate::parser::choice
// Provides: {"impl_418"}
// Dependencies: {}
impl < Input , O , P > ChoiceParser < Input > for [P] where Input : Stream , P : Parser < Input , Output = O > , { type Output = O ; type PartialState = (usize , P :: PartialState) ; # [inline] fn parse_partial (& mut self , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { slice_parse_mode (self , crate :: parser :: PartialMode :: default () , input , state) } # [inline] fn parse_first (& mut self , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { slice_parse_mode (self , crate :: parser :: FirstMode , input , state) } # [inline] fn parse_mode_choice < M > (& mut self , _mode : M , _input : & mut Input , _state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { unreachable ! () } fn add_error_choice (& mut self , error : & mut Tracked < < Input as StreamOnce > :: Error >) { if error . offset != ErrorOffset (0) { for p in self { error . offset = ErrorOffset (1) ; p . add_error (error) ; } } } }
};
}
