macro_rules! deps {
    () => {
        CastToUnsized!();
        TraitImplOrphan!();
        MissingFields!();
        TypedHole!();
        UnresolvedExternCrate!();
        MissingLifetime!();
        MacroDefError!();
        AwaitOutsideOfAsync!();
        UnresolvedMethodCall!();
        IncorrectGenericsOrder!();
        PrivateField!();
        MismatchedTupleStructPatArgCount!();
        UnusedMut!();
        ElidedLifetimesInPath!();
        BreakOutsideOfLoop!();
        BadRtn!();
        NonExhaustiveLet!();
        UnresolvedImport!();
        PrivateAssocItem!();
        IncoherentImpl!();
        TypeMismatch!();
        NeedMut!();
        ExpectedFunction!();
        InvalidDeriveTarget!();
        MalformedDerive!();
        TraitImplRedundantAssocItems!();
        UnimplementedBuiltinMacro!();
        UnresolvedField!();
        MacroExpansionParseError!();
        UnresolvedAssocItem!();
        TraitImplIncorrectSafety!();
        InactiveCode!();
        NoSuchField!();
        UndeclaredLabel!();
        MismatchedArgCount!();
        MissingMatchArms!();
        MissingUnsafe!();
        RemoveTrailingReturn!();
        UnresolvedMacroCall!();
        MacroError!();
        UnresolvedModule!();
        UnresolvedIdent!();
        UnusedVariable!();
        GenericArgsProhibited!();
        RemoveUnnecessaryElse!();
        ReplaceFilterMapNextWithFindMap!();
        TraitImplMissingAssocItems!();
        ParenthesizedGenericArgsWithoutFnTrait!();
        IncorrectGenericsLen!();
        InvalidCast!();
        MovedOutOfRef!();
        UnreachableLabel!();
    };
}

macro_rules! macro_115 {
    () => {
        deps!();
        diagnostics ! [AnyDiagnostic <'db > -> AwaitOutsideOfAsync , BreakOutsideOfLoop , CastToUnsized <'db >, ExpectedFunction <'db >, InactiveCode , IncoherentImpl , IncorrectCase , InvalidCast <'db >, InvalidDeriveTarget , MacroDefError , MacroError , MacroExpansionParseError , MalformedDerive , MismatchedArgCount , MismatchedTupleStructPatArgCount , MissingFields , MissingMatchArms , MissingUnsafe , MovedOutOfRef <'db >, NeedMut , NonExhaustiveLet , NoSuchField , PrivateAssocItem , PrivateField , RemoveTrailingReturn , RemoveUnnecessaryElse , ReplaceFilterMapNextWithFindMap , TraitImplIncorrectSafety , TraitImplMissingAssocItems , TraitImplOrphan , TraitImplRedundantAssocItems , TypedHole <'db >, TypeMismatch <'db >, UndeclaredLabel , UnimplementedBuiltinMacro , UnreachableLabel , UnresolvedAssocItem , UnresolvedExternCrate , UnresolvedField <'db >, UnresolvedImport , UnresolvedMacroCall , UnresolvedMethodCall <'db >, UnresolvedModule , UnresolvedIdent , UnusedMut , UnusedVariable , GenericArgsProhibited , ParenthesizedGenericArgsWithoutFnTrait , BadRtn , IncorrectGenericsLen , IncorrectGenericsOrder , MissingLifetime , ElidedLifetimesInPath ,] ;
    };
}

macro_115!()