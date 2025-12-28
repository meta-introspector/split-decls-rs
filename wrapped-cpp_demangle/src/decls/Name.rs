macro_rules! deps {
    () => {
        NestedName!();
        TemplateArgs!();
        LocalName!();
        UnscopedName!();
    };
}

macro_rules! Name {
    () => {
        deps!();
        # [doc = " The `<name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <name> ::= <nested-name>"] # [doc = "        ::= <unscoped-name>"] # [doc = "        ::= <unscoped-template-name> <template-args>"] # [doc = "        ::= <local-name>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum Name { # [doc = " A nested name"] Nested (NestedName) , # [doc = " An unscoped name."] Unscoped (UnscopedName) , # [doc = " An unscoped template."] UnscopedTemplate (UnscopedTemplateNameHandle , TemplateArgs) , # [doc = " A local name."] Local (LocalName) , }
    };
}

Name!()