// Generated macro for tests (module)
macro_rules! Depcrate_validatortests {
() => {
// Module: crate::validator
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: parser :: { consume_rules , PestParser } ; use super :: super :: unwrap_or_report ; use super :: * ; use pest :: Parser ; # [test] # [should_panic (expected = "grammar error

 --> 1:1
  |
1 | ANY = { \"a\" }
  | ^-^
  |
  = ANY is a pest keyword")] fn pest_keyword () { let input = "ANY = { \"a\" }" ; unwrap_or_report (validate_pairs (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:13
  |
1 | a = { \"a\" } a = { \"a\" }
  |             ^
  |
  = rule a already defined")] fn already_defined () { let input = "a = { \"a\" } a = { \"a\" }" ; unwrap_or_report (validate_pairs (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:7
  |
1 | a = { b }
  |       ^
  |
  = rule b is undefined")] fn undefined () { let input = "a = { b }" ; unwrap_or_report (validate_pairs (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] fn valid_recursion () { let input = "a = { \"\" ~ \"a\"? ~ \"a\"* ~ (\"a\" | \"b\") ~ a }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:16
  |
1 | WHITESPACE = { \"\" }
  |                ^^
  |
  = WHITESPACE cannot fail and will repeat infinitely")] fn non_failing_whitespace () { let input = "WHITESPACE = { \"\" }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:13
  |
1 | COMMENT = { SOI }
  |             ^-^
  |
  = COMMENT is non-progressing and will repeat infinitely")] fn non_progressing_comment () { let input = "COMMENT = { SOI }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] fn non_progressing_empty_string () { assert ! (is_non_failing (& ParserExpr :: Insens ("" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: Str ("" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn progressing_non_empty_string () { assert ! (! is_non_progressing (& ParserExpr :: Insens ("non empty" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_progressing (& ParserExpr :: Str ("non empty" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn non_progressing_soi_eoi () { assert ! (is_non_progressing (& ParserExpr :: Ident ("SOI" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: Ident ("EOI" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn non_progressing_predicates () { let progressing = ParserExpr :: Str ("A" . into ()) ; assert ! (is_non_progressing (& ParserExpr :: PosPred (Box :: new (ParserNode { expr : progressing . clone () , span : Span :: new (" " , 0 , 1) . unwrap () , })) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: NegPred (Box :: new (ParserNode { expr : progressing , span : Span :: new (" " , 0 , 1) . unwrap () , })) , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn non_progressing_0_length_repetitions () { let input_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("A" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (! is_non_progressing (& input_progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: Rep (input_progressing_node . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: Opt (input_progressing_node . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: RepExact (input_progressing_node . clone () , 0) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: RepMin (input_progressing_node . clone () , 0) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: RepMax (input_progressing_node . clone () , 0) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: RepMax (input_progressing_node . clone () , 17) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: RepMinMax (input_progressing_node . clone () , 0 , 12) , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn non_progressing_nonzero_repetitions_with_non_progressing_expr () { let a = "" ; let non_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str (a . into ()) , span : Span :: new (a , 0 , 0) . unwrap () , }) ; let exact = ParserExpr :: RepExact (non_progressing_node . clone () , 7) ; let min = ParserExpr :: RepMin (non_progressing_node . clone () , 23) ; let minmax = ParserExpr :: RepMinMax (non_progressing_node . clone () , 12 , 13) ; let reponce = ParserExpr :: RepOnce (non_progressing_node) ; assert ! (is_non_progressing (& exact , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& min , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& minmax , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& reponce , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn progressing_repetitions () { let a = "A" ; let input_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str (a . into ()) , span : Span :: new (a , 0 , 1) . unwrap () , }) ; let exact = ParserExpr :: RepExact (input_progressing_node . clone () , 1) ; let min = ParserExpr :: RepMin (input_progressing_node . clone () , 2) ; let minmax = ParserExpr :: RepMinMax (input_progressing_node . clone () , 4 , 5) ; let reponce = ParserExpr :: RepOnce (input_progressing_node) ; assert ! (! is_non_progressing (& exact , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_progressing (& min , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_progressing (& minmax , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_progressing (& reponce , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn non_progressing_push () { let a = "" ; let non_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str (a . into ()) , span : Span :: new (a , 0 , 0) . unwrap () , }) ; let push = ParserExpr :: Push (non_progressing_node . clone ()) ; assert ! (is_non_progressing (& push , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn progressing_push () { let a = "i'm make progress" ; let progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str (a . into ()) , span : Span :: new (a , 0 , 1) . unwrap () , }) ; let push = ParserExpr :: Push (progressing_node . clone ()) ; assert ! (! is_non_progressing (& push , & HashMap :: new () , & mut Vec :: new ())) ; } # [cfg (feature = "grammar-extras")] # [test] fn push_literal_is_non_progressing () { let a = "" ; let non_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: PushLiteral ("a" . to_string ()) , span : Span :: new (a , 0 , 0) . unwrap () , }) ; let push = ParserExpr :: Push (non_progressing_node . clone ()) ; assert ! (is_non_progressing (& push , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn node_tag_forwards_is_non_progressing () { let progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("i'm make progress" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (! is_non_progressing (& progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let non_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_progressing (& non_progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; # [cfg (feature = "grammar-extras")] { let progressing = ParserExpr :: NodeTag (progressing_node . clone () , "TAG" . into ()) ; let non_progressing = ParserExpr :: NodeTag (non_progressing_node . clone () , "TAG" . into ()) ; assert ! (! is_non_progressing (& progressing , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& non_progressing , & HashMap :: new () , & mut Vec :: new ())) ; } } # [test] fn progressing_range () { let progressing = ParserExpr :: Range ("A" . into () , "Z" . into ()) ; let failing_is_progressing = ParserExpr :: Range ("Z" . into () , "A" . into ()) ; assert ! (! is_non_progressing (& progressing , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_progressing (& failing_is_progressing , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn progressing_choice () { let left_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("i'm make progress" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (! is_non_progressing (& left_progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let right_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Ident ("DROP" . into ()) , span : Span :: new ("DROP" , 0 , 3) . unwrap () , }) ; assert ! (! is_non_progressing (& ParserExpr :: Choice (left_progressing_node , right_progressing_node) , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn non_progressing_choices () { let left_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("i'm make progress" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (! is_non_progressing (& left_progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let left_non_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_progressing (& left_non_progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let right_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Ident ("DROP" . into ()) , span : Span :: new ("DROP" , 0 , 3) . unwrap () , }) ; assert ! (! is_non_progressing (& right_progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let right_non_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Opt (Box :: new (ParserNode { expr : ParserExpr :: Str ("   " . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , })) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_progressing (& right_non_progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: Choice (left_non_progressing_node . clone () , right_progressing_node) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: Choice (left_progressing_node , right_non_progressing_node . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_progressing (& ParserExpr :: Choice (left_non_progressing_node , right_non_progressing_node) , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn non_progressing_seq () { let left_non_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; let right_non_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Opt (Box :: new (ParserNode { expr : ParserExpr :: Str ("   " . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , })) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_progressing (& ParserExpr :: Seq (left_non_progressing_node , right_non_progressing_node) , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn progressing_seqs () { let left_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("i'm make progress" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (! is_non_progressing (& left_progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let left_non_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_progressing (& left_non_progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let right_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Ident ("DROP" . into ()) , span : Span :: new ("DROP" , 0 , 3) . unwrap () , }) ; assert ! (! is_non_progressing (& right_progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let right_non_progressing_node = Box :: new (ParserNode { expr : ParserExpr :: Opt (Box :: new (ParserNode { expr : ParserExpr :: Str ("   " . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , })) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_progressing (& right_non_progressing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_progressing (& ParserExpr :: Seq (left_non_progressing_node , right_progressing_node . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_progressing (& ParserExpr :: Seq (left_progressing_node . clone () , right_non_progressing_node) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_progressing (& ParserExpr :: Seq (left_progressing_node , right_progressing_node) , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn progressing_stack_operations () { assert ! (! is_non_progressing (& ParserExpr :: Ident ("DROP" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_progressing (& ParserExpr :: Ident ("PEEK" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_progressing (& ParserExpr :: Ident ("POP" . into ()) , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn non_failing_string () { let insens = ParserExpr :: Insens ("" . into ()) ; let string = ParserExpr :: Str ("" . into ()) ; assert ! (is_non_failing (& insens , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& string , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn failing_string () { assert ! (! is_non_failing (& ParserExpr :: Insens ("i may fail!" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& ParserExpr :: Str ("failure is not fatal" . into ()) , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn failing_stack_operations () { assert ! (! is_non_failing (& ParserExpr :: Ident ("DROP" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& ParserExpr :: Ident ("POP" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& ParserExpr :: Ident ("PEEK" . into ()) , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn non_failing_zero_length_repetitions () { let failing = Box :: new (ParserNode { expr : ParserExpr :: Range ("A" . into () , "B" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (! is_non_failing (& failing . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: Opt (failing . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: Rep (failing . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: RepExact (failing . clone () , 0) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: RepMin (failing . clone () , 0) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: RepMax (failing . clone () , 0) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: RepMax (failing . clone () , 22) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: RepMinMax (failing . clone () , 0 , 73) , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn non_failing_non_zero_repetitions_with_non_failing_expr () { let non_failing = Box :: new (ParserNode { expr : ParserExpr :: Opt (Box :: new (ParserNode { expr : ParserExpr :: Range ("A" . into () , "B" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , })) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_failing (& non_failing . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: RepOnce (non_failing . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: RepExact (non_failing . clone () , 1) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: RepMin (non_failing . clone () , 6) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: RepMinMax (non_failing . clone () , 32 , 73) , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] # [cfg (feature = "grammar-extras")] fn failing_non_zero_repetitions () { let failing = Box :: new (ParserNode { expr : ParserExpr :: NodeTag (Box :: new (ParserNode { expr : ParserExpr :: Range ("A" . into () , "B" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) , "Tag" . into () ,) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (! is_non_failing (& failing . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& ParserExpr :: RepOnce (failing . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& ParserExpr :: RepExact (failing . clone () , 3) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& ParserExpr :: RepMin (failing . clone () , 14) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& ParserExpr :: RepMinMax (failing . clone () , 47 , 73) , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn failing_choice () { let left_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("i'm a failure" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (! is_non_failing (& left_failing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let right_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Ident ("DROP" . into ()) , span : Span :: new ("DROP" , 0 , 3) . unwrap () , }) ; assert ! (! is_non_failing (& ParserExpr :: Choice (left_failing_node , right_failing_node) , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn non_failing_choices () { let left_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("i'm a failure" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (! is_non_failing (& left_failing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let left_non_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_failing (& left_non_failing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let right_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Ident ("DROP" . into ()) , span : Span :: new ("DROP" , 0 , 3) . unwrap () , }) ; assert ! (! is_non_failing (& right_failing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let right_non_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Opt (Box :: new (ParserNode { expr : ParserExpr :: Str ("   " . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , })) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_failing (& right_non_failing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: Choice (left_non_failing_node . clone () , right_failing_node) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: Choice (left_failing_node , right_non_failing_node . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: Choice (left_non_failing_node , right_non_failing_node) , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn non_failing_seq () { let left_non_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; let right_non_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Opt (Box :: new (ParserNode { expr : ParserExpr :: Str ("   " . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , })) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_failing (& ParserExpr :: Seq (left_non_failing_node , right_non_failing_node) , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn failing_seqs () { let left_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("i'm a failure" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (! is_non_failing (& left_failing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let left_non_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_failing (& left_non_failing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let right_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Ident ("DROP" . into ()) , span : Span :: new ("DROP" , 0 , 3) . unwrap () , }) ; assert ! (! is_non_failing (& right_failing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let right_non_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Opt (Box :: new (ParserNode { expr : ParserExpr :: Str ("   " . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , })) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_failing (& right_non_failing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& ParserExpr :: Seq (left_non_failing_node , right_failing_node . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& ParserExpr :: Seq (left_failing_node . clone () , right_non_failing_node) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& ParserExpr :: Seq (left_failing_node , right_failing_node) , & HashMap :: new () , & mut Vec :: new ())) } # [test] fn failing_range () { let failing = ParserExpr :: Range ("A" . into () , "Z" . into ()) ; let always_failing = ParserExpr :: Range ("Z" . into () , "A" . into ()) ; assert ! (! is_non_failing (& failing , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& always_failing , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] fn _push_node_tag_pos_pred_forwarding_is_non_failing () { let failing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("i'm a failure" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (! is_non_failing (& failing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; let non_failing_node = Box :: new (ParserNode { expr : ParserExpr :: Str ("" . into ()) , span : Span :: new (" " , 0 , 1) . unwrap () , }) ; assert ! (is_non_failing (& non_failing_node . clone () . expr , & HashMap :: new () , & mut Vec :: new ())) ; # [cfg (feature = "grammar-extras")] { assert ! (! is_non_failing (& ParserExpr :: NodeTag (failing_node . clone () , "TAG" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: NodeTag (non_failing_node . clone () , "TAG" . into ()) , & HashMap :: new () , & mut Vec :: new ())) ; } assert ! (! is_non_failing (& ParserExpr :: Push (failing_node . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: Push (non_failing_node . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (! is_non_failing (& ParserExpr :: PosPred (failing_node . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; assert ! (is_non_failing (& ParserExpr :: PosPred (non_failing_node . clone ()) , & HashMap :: new () , & mut Vec :: new ())) ; } # [cfg (feature = "grammar-extras")] # [test] fn push_literal_is_non_failing () { assert ! (is_non_failing (& ParserExpr :: PushLiteral ("a" . to_string ()) , & HashMap :: new () , & mut Vec :: new ())) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:7
  |
1 | a = { (\"\")* }
  |       ^---^
  |
  = expression inside repetition cannot fail and will repeat infinitely")] fn non_failing_repetition () { let input = "a = { (\"\")* }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:18
  |
1 | a = { \"\" } b = { a* }
  |                  ^^
  |
  = expression inside repetition cannot fail and will repeat infinitely")] fn indirect_non_failing_repetition () { let input = "a = { \"\" } b = { a* }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:20
  |
1 | a = { \"a\" ~ (\"b\" ~ (\"\")*) }
  |                    ^---^
  |
  = expression inside repetition cannot fail and will repeat infinitely")] fn deep_non_failing_repetition () { let input = "a = { \"a\" ~ (\"b\" ~ (\"\")*) }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:7
  |
1 | a = { (\"\" ~ &\"a\" ~ !\"a\" ~ (SOI | EOI))* }
  |       ^-------------------------------^
  |
  = expression inside repetition is non-progressing and will repeat infinitely")] fn non_progressing_repetition () { let input = "a = { (\"\" ~ &\"a\" ~ !\"a\" ~ (SOI | EOI))* }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:20
  |
1 | a = { !\"a\" } b = { a* }
  |                    ^^
  |
  = expression inside repetition is non-progressing and will repeat infinitely")] fn indirect_non_progressing_repetition () { let input = "a = { !\"a\" } b = { a* }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:7
  |
1 | a = { a }
  |       ^
  |
  = rule a is left-recursive (a -> a); pest::pratt_parser might be useful in this case")] fn simple_left_recursion () { let input = "a = { a }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:7
  |
1 | a = { b } b = { a }
  |       ^
  |
  = rule b is left-recursive (b -> a -> b); pest::pratt_parser might be useful in this case

 --> 1:17
  |
1 | a = { b } b = { a }
  |                 ^
  |
  = rule a is left-recursive (a -> b -> a); pest::pratt_parser might be useful in this case")] fn indirect_left_recursion () { let input = "a = { b } b = { a }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:39
  |
1 | a = { \"\" ~ \"a\"? ~ \"a\"* ~ (\"a\" | \"\") ~ a }
  |                                       ^
  |
  = rule a is left-recursive (a -> a); pest::pratt_parser might be useful in this case")] fn non_failing_left_recursion () { let input = "a = { \"\" ~ \"a\"? ~ \"a\"* ~ (\"a\" | \"\") ~ a }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:13
  |
1 | a = { \"a\" | a }
  |             ^
  |
  = rule a is left-recursive (a -> a); pest::pratt_parser might be useful in this case")] fn non_primary_choice_left_recursion () { let input = "a = { \"a\" | a }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:14
  |
1 | a = { !\"a\" ~ a }
  |              ^
  |
  = rule a is left-recursive (a -> a); pest::pratt_parser might be useful in this case")] fn non_progressing_left_recursion () { let input = "a = { !\"a\" ~ a }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:7
  |
1 | a = { \"a\"* | \"a\" | \"b\" }
  |       ^--^
  |
  = expression cannot fail; following choices cannot be reached")] fn lhs_non_failing_choice () { let input = "a = { \"a\"* | \"a\" | \"b\" }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:13
  |
1 | a = { \"a\" | \"a\"* | \"b\" }
  |             ^--^
  |
  = expression cannot fail; following choices cannot be reached")] fn lhs_non_failing_choice_middle () { let input = "a = { \"a\" | \"a\"* | \"b\" }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:7
  |
1 | a = { b | \"a\" } b = { \"b\"* | \"c\" }
  |       ^
  |
  = expression cannot fail; following choices cannot be reached

 --> 1:23
  |
1 | a = { b | \"a\" } b = { \"b\"* | \"c\" }
  |                       ^--^
  |
  = expression cannot fail; following choices cannot be reached")] fn lhs_non_failing_nested_choices () { let input = "a = { b | \"a\" } b = { \"b\"* | \"c\" }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] fn skip_can_be_defined () { let input = "skip = { \"\" }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:7
  |
1 | a = { #b = b } b = _{ ASCII_DIGIT+ }
  |       ^----^
  |
  = tags on silent rules will not appear in the output")] # [cfg (feature = "grammar-extras")] fn tag_on_silent_rule () { let input = "a = { #b = b } b = _{ ASCII_DIGIT+ }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [should_panic (expected = "grammar error

 --> 1:7
  |
1 | a = { #b = ASCII_DIGIT+ }
  |       ^---------------^
  |
  = tags on built-in rules will not appear in the output")] # [cfg (feature = "grammar-extras")] fn tag_on_builtin_rule () { let input = "a = { #b = ASCII_DIGIT+ }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } # [test] # [cfg (feature = "grammar-extras")] fn tag_on_normal_rule () { let input = "a = { #b = b } b = { ASCII_DIGIT+ }" ; unwrap_or_report (consume_rules (PestParser :: parse (Rule :: grammar_rules , input) . unwrap () ,)) ; } }
};
}
