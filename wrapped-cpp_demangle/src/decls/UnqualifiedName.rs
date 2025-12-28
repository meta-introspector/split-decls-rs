macro_rules! deps {
    () => {
        UnnamedTypeName!();
        AbiTags!();
        Discriminator!();
        CtorDtorName!();
        SourceName!();
        OperatorName!();
        ClosureTypeName!();
    };
}

macro_rules! UnqualifiedName {
    () => {
        deps!();
        # [doc = " The `<unqualified-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <unqualified-name> ::= [on] <operator-name> [<abi-tags>]"] # [doc = "                    ::= <ctor-dtor-name> [<abi-tags>]"] # [doc = "                    ::= <source-name> [<abi-tags>]"] # [doc = "                    ::= <local-source-name> [<abi-tags>]"] # [doc = "                    ::= <unnamed-type-name> [<abi-tags>]"] # [doc = "                    ::= <closure-type-name> [<abi-tags>]"] # [doc = ""] # [doc = " # I think this is from an older version of the standard. It isn't in the"] # [doc = " # current version, but all the other demanglers support it, so we will too."] # [doc = " <local-source-name> ::= L <source-name> [<discriminator>]"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum UnqualifiedName { # [doc = " An operator name."] Operator (OperatorName , AbiTags) , # [doc = " A constructor or destructor name."] CtorDtor (CtorDtorName , AbiTags) , # [doc = " A source name."] Source (SourceName , AbiTags) , # [doc = " A local source name."] LocalSourceName (SourceName , Option < Discriminator > , AbiTags) , # [doc = " A generated name for an unnamed type."] UnnamedType (UnnamedTypeName , AbiTags) , # [doc = " A closure type name"] ClosureType (ClosureTypeName , AbiTags) , }
    };
}

UnqualifiedName!();