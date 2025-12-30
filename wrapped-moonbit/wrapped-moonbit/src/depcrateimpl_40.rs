// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl ToMoonBitIdent for str { fn to_moonbit_ident (& self) -> String { match self { "as" | "else" | "extern" | "fn" | "fnalias" | "if" | "let" | "const" | "match" | "using" | "mut" | "type" | "typealias" | "struct" | "enum" | "trait" | "traitalias" | "derive" | "while" | "break" | "continue" | "import" | "return" | "throw" | "raise" | "try" | "catch" | "pub" | "priv" | "readonly" | "true" | "false" | "_" | "test" | "loop" | "for" | "in" | "impl" | "with" | "guard" | "async" | "is" | "suberror" | "and" | "letrec" | "enumview" | "noraise" | "defer" | "init" | "main" | "module" | "move" | "ref" | "static" | "super" | "unsafe" | "use" | "where" | "await" | "dyn" | "abstract" | "do" | "final" | "macro" | "override" | "typeof" | "virtual" | "yield" | "local" | "method" | "alias" | "assert" | "package" | "recur" | "isnot" | "define" | "downcast" | "inherit" | "member" | "namespace" | "upcast" | "void" | "lazy" | "include" | "mixin" | "protected" | "sealed" | "constructor" | "atomic" | "volatile" | "anyframe" | "anytype" | "asm" | "comptime" | "errdefer" | "export" | "opaque" | "orelse" | "resume" | "threadlocal" | "unreachable" | "dynclass" | "dynobj" | "dynrec" | "var" | "finally" | "noasync" => { format ! ("{self}_") } _ => self . strip_prefix ("[async]") . unwrap_or (self) . to_snake_case () , } } }
};
}
