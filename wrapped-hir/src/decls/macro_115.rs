macro_rules! deps {
    () => {
        PrivateAssocItem!();
        TraitImplOrphan!();
        UndeclaredLabel!();
        RemoveUnnecessaryElse!();
        ReplaceFilterMapNextWithFindMap!();
        UnreachableLabel!();
        UnresolvedMacroCall!();
        ParenthesizedGenericArgsWithoutFnTrait!();
        MacroExpansionParseError!();
        MovedOutOfRef!();
        TraitImplRedundantAssocItems!();
        UnresolvedField!();
        PrivateField!();
        InactiveCode!();
        UnresolvedIdent!();
        TypeMismatch!();
        UnresolvedImport!();
        UnusedMut!();
        ElidedLifetimesInPath!();
        MalformedDerive!();
        UnresolvedAssocItem!();
        MissingMatchArms!();
        IncorrectGenericsLen!();
        MacroDefError!();
        NonExhaustiveLet!();
        CastToUnsized!();
        MacroError!();
        NeedMut!();
        TraitImplMissingAssocItems!();
        UnresolvedMethodCall!();
        IncoherentImpl!();
        InvalidCast!();
        NoSuchField!();
        MismatchedTupleStructPatArgCount!();
        UnresolvedModule!();
        UnresolvedExternCrate!();
        GenericArgsProhibited!();
        BadRtn!();
        AwaitOutsideOfAsync!();
        RemoveTrailingReturn!();
        MissingLifetime!();
        BreakOutsideOfLoop!();
        TraitImplIncorrectSafety!();
        InvalidDeriveTarget!();
        MismatchedArgCount!();
        MissingUnsafe!();
        TypedHole!();
        UnusedVariable!();
        MissingFields!();
        IncorrectGenericsOrder!();
        ExpectedFunction!();
        UnimplementedBuiltinMacro!();
    };
}

macro_rules! macro_115 {
    () => {
        deps!();
        diagnostics ! [AnyDiagnostic <'db > -> AwaitOutsideOfAsync , BreakOutsideOfLoop , CastToUnsized <'db >, ExpectedFunction <'db >, InactiveCode , IncoherentImpl , IncorrectCase , InvalidCast <'db >, InvalidDeriveTarget , MacroDefError , MacroError , MacroExpansionParseError , MalformedDerive , MismatchedArgCount , MismatchedTupleStructPatArgCount , MissingFields , MissingMatchArms , MissingUnsafe , MovedOutOfRef <'db >, NeedMut , NonExhaustiveLet , NoSuchField , PrivateAssocItem , PrivateField , RemoveTrailingReturn , RemoveUnnecessaryElse , ReplaceFilterMapNextWithFindMap , TraitImplIncorrectSafety , TraitImplMissingAssocItems , TraitImplOrphan , TraitImplRedundantAssocItems , TypedHole <'db >, TypeMismatch <'db >, UndeclaredLabel , UnimplementedBuiltinMacro , UnreachableLabel , UnresolvedAssocItem , UnresolvedExternCrate , UnresolvedField <'db >, UnresolvedImport , UnresolvedMacroCall , UnresolvedMethodCall <'db >, UnresolvedModule , UnresolvedIdent , UnusedMut , UnusedVariable , GenericArgsProhibited , ParenthesizedGenericArgsWithoutFnTrait , BadRtn , IncorrectGenericsLen , IncorrectGenericsOrder , MissingLifetime , ElidedLifetimesInPath ,] ;
    };
}

macro_115!();