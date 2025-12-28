macro_rules! deps {
    () => {
        Decltype!();
        TemplateParam!();
        DataMemberPrefix!();
        UnqualifiedName!();
        TemplateArgs!();
    };
}

macro_rules! Prefix {
    () => {
        deps!();
        # [doc = " The `<prefix>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <prefix> ::= <unqualified-name>"] # [doc = "          ::= <prefix> <unqualified-name>"] # [doc = "          ::= <template-prefix> <template-args>"] # [doc = "          ::= <template-param>"] # [doc = "          ::= <decltype>"] # [doc = "          ::= <prefix> <data-member-prefix>"] # [doc = "          ::= <substitution>"] # [doc = ""] # [doc = " <template-prefix> ::= <template unqualified-name>"] # [doc = "                   ::= <prefix> <template unqualified-name>"] # [doc = "                   ::= <template-param>"] # [doc = "                   ::= <substitution>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum Prefix { # [doc = " An unqualified name."] Unqualified (UnqualifiedName) , # [doc = " Some nested name."] Nested (PrefixHandle , UnqualifiedName) , # [doc = " A prefix and template arguments."] Template (PrefixHandle , TemplateArgs) , # [doc = " A template parameter."] TemplateParam (TemplateParam) , # [doc = " A decltype."] Decltype (Decltype) , # [doc = " A prefix and data member."] DataMember (PrefixHandle , DataMemberPrefix) , }
    };
}

Prefix!()