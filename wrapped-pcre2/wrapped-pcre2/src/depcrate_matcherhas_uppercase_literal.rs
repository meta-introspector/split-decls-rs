// Generated macro for has_uppercase_literal (function)
macro_rules! Depcrate_matcherhas_uppercase_literal {
() => {
// Module: crate::matcher
// Provides: {"has_uppercase_literal"}
// Dependencies: {}
# [doc = " Determine whether the pattern contains an uppercase character which should"] # [doc = " negate the effect of the smart-case option."] # [doc = ""] # [doc = " Ideally we would be able to check the AST in order to correctly handle"] # [doc = " things like '\\p{Ll}' and '\\p{Lu}' (which should be treated as explicitly"] # [doc = " cased), but PCRE doesn't expose enough details for that kind of analysis."] # [doc = " For now, our 'good enough' solution is to simply perform a semi-naïve"] # [doc = " scan of the input pattern and ignore all characters following a '\\'. The"] # [doc = " This at least lets us support the most common cases, like 'foo\\w' and"] # [doc = " 'foo\\S', in an intuitive manner."] fn has_uppercase_literal (pattern : & str) -> bool { let mut chars = pattern . chars () ; while let Some (c) = chars . next () { if c == '\\' { chars . next () ; } else if c . is_uppercase () { return true ; } } false }
};
}
