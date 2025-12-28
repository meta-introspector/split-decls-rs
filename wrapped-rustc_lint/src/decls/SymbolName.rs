macro_rules! SymbolName {
    () => {
        # [doc = " Differentiate between whether the name for an extern decl came from the link_name attribute or"] # [doc = " just from declaration itself. This is important because we don't want to report clashes on"] # [doc = " symbol name if they don't actually clash because one or the other links against a symbol with a"] # [doc = " different name."] enum SymbolName { # [doc = " The name of the symbol + the span of the annotation which introduced the link name."] Link (Symbol , Span) , # [doc = " No link name, so just the name of the symbol."] Normal (Symbol) , }
    };
}

SymbolName!();