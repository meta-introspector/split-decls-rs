macro_rules! deps {
    () => {
        Moniker!();
        MonikerResult!();
    };
}

macro_rules! def_to_moniker {
    () => {
        deps!();
        # [doc = " Computes a `MonikerResult` for a definition. Result cases:"] # [doc = ""] # [doc = " * `Some(MonikerResult::Moniker(_))` provides a unique `Moniker` which refers to a definition."] # [doc = ""] # [doc = " * `Some(MonikerResult::Local { .. })` provides a `Moniker` for the definition enclosing a local."] # [doc = ""] # [doc = " * `None` is returned for definitions which are not in a module: `BuiltinAttr`, `BuiltinType`,"] # [doc = "   `BuiltinLifetime`, `TupleField`, `ToolModule`, and `InlineAsmRegOrRegClass`. TODO: it might be"] # [doc = "   sensible to provide monikers that refer to some non-existent crate of compiler builtin"] # [doc = "   definitions."] pub (crate) fn def_to_moniker (db : & RootDatabase , definition : Definition , from_crate : Crate ,) -> Option < MonikerResult > { match definition { Definition :: Local (_) | Definition :: Label (_) | Definition :: GenericParam (_) => { return Some (MonikerResult :: Local { enclosing_moniker : enclosing_def_to_moniker (db , definition , from_crate) , }) ; } _ => { } } Some (MonikerResult :: Moniker (def_to_non_local_moniker (db , definition , from_crate) ?)) }
    };
}

def_to_moniker!();