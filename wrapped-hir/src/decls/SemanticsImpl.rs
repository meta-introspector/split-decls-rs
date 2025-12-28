macro_rules! SemanticsImpl {
    () => {
        pub struct SemanticsImpl < 'db > { pub db : & 'db dyn HirDatabase , s2d_cache : RefCell < SourceToDefCache > , # [doc = " MacroCall to its expansion's MacroCallId cache"] macro_call_cache : RefCell < FxHashMap < InFile < ast :: MacroCall > , MacroCallId > > , }
    };
}

SemanticsImpl!()