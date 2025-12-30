// Generated macro for ExpressionKind (enum)
macro_rules! Depcrate_grammarExpressionKind {
() => {
// Module: crate::grammar
// Provides: {"ExpressionKind"}
// Dependencies: {}
# [derive (Clone , Debug)] enum ExpressionKind { # [doc = " `( A B C )`"] Grouped (Box < Expression >) , # [doc = " `A | B | C`"] Alt (Vec < Expression >) , # [doc = " `A B C`"] Sequence (Vec < Expression >) , # [doc = " `A?`"] Optional (Box < Expression >) , # [doc = " `A*`"] Repeat (Box < Expression >) , # [doc = " `A*?`"] RepeatNonGreedy (Box < Expression >) , # [doc = " `A+`"] RepeatPlus (Box < Expression >) , # [doc = " `A+?`"] RepeatPlusNonGreedy (Box < Expression >) , # [doc = " `A{2..4}`"] RepeatRange (Box < Expression > , Option < u32 > , Option < u32 >) , # [doc = " `NonTerminal`"] Nt (String) , # [doc = " `` `string` ``"] Terminal (String) , # [doc = " `<english description>`"] Prose (String) , # [doc = " An LF followed by the given number of spaces."] # [doc = ""] # [doc = " Used by the renderer to help format and structure the grammar."] Break (usize) , # [doc = " `// Single line comment.`"] Comment (String) , # [doc = " ``[`A`-`Z` `_` LF]``"] Charset (Vec < Characters >) , # [doc = " ``~[` ` LF]``"] NegExpression (Box < Expression >) , # [doc = " `U+0060`"] Unicode (String) , }
};
}
