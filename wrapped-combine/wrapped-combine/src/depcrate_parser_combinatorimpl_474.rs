// Generated macro for impl_474 (impl)
macro_rules! Depcrate_parser_combinatorimpl_474 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_474"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < Input , P > Parser < Input > for AnySendPartialStateParser < P > where Input : Stream , P : Parser < Input > , P :: PartialState : Send + 'static , { type Output = P :: Output ; type PartialState = AnySendPartialState ; # [inline] fn parse_lazy (& mut self , input : & mut Input ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { self . 0 . parse_lazy (input) } parse_mode ! (Input) ; # [inline] fn parse_mode < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { let mut new_child_state ; let result = { let child_state = if state . 0 . is_none () { new_child_state = Some (Default :: default ()) ; new_child_state . as_mut () . unwrap () } else { new_child_state = None ; state . 0 . as_mut () . unwrap () . downcast_mut () . unwrap () } ; self . 0 . parse_mode (mode , input , child_state) } ; if let CommitErr (_) = result { if state . 0 . is_none () { state . 0 = Some (Box :: new (new_child_state . unwrap ())) ; } } result } forward_parser ! (Input , add_error add_committed_expected_error parser_count , 0) ; }
};
}
