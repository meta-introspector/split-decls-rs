macro_rules! deps {
    () => {
        TomlDebugInfo!();
        Result!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl Display for TomlDebugInfo { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TomlDebugInfo :: None => f . write_char ('0') , TomlDebugInfo :: Limited => f . write_char ('1') , TomlDebugInfo :: Full => f . write_char ('2') , TomlDebugInfo :: LineDirectivesOnly => f . write_str ("line-directives-only") , TomlDebugInfo :: LineTablesOnly => f . write_str ("line-tables-only") , } } }
    };
}

impl_132!();