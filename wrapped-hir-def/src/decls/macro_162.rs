macro_rules! deps {
    () => {
        Macro2!();
        ExternCrate!();
        Trait!();
        Const!();
        TypeAlias!();
        Union!();
        Function!();
        MacroCall!();
        Enum!();
        MacroRules!();
        Struct!();
        Impl!();
        ExternBlock!();
        Static!();
        Use!();
        Mod!();
    };
}

macro_rules! macro_162 {
    () => {
        deps!();
        mod_items ! { ModItemId -> Const in small_data -> ast :: Const , Enum in small_data -> ast :: Enum , ExternBlock in small_data -> ast :: ExternBlock , ExternCrate in big_data -> ast :: ExternCrate , Function in small_data -> ast :: Fn , Impl in small_data -> ast :: Impl , Macro2 in small_data -> ast :: MacroDef , MacroCall in small_data -> ast :: MacroCall , MacroRules in small_data -> ast :: MacroRules , Mod in big_data -> ast :: Module , Static in small_data -> ast :: Static , Struct in small_data -> ast :: Struct , Trait in small_data -> ast :: Trait , TypeAlias in small_data -> ast :: TypeAlias , Union in small_data -> ast :: Union , Use in big_data -> ast :: Use , }
    };
}

macro_162!()