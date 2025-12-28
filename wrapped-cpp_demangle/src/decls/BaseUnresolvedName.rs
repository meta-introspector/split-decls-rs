macro_rules! deps {
    () => {
        TemplateArgs!();
        Name!();
        DestructorName!();
        OperatorName!();
        SimpleId!();
    };
}

macro_rules! BaseUnresolvedName {
    () => {
        deps!();
        # [doc = " The `<base-unresolved-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <base-unresolved-name> ::= <simple-id>                        # unresolved name"] # [doc = "                        ::= on <operator-name>                 # unresolved operator-function-id"] # [doc = "                        ::= on <operator-name> <template-args> # unresolved operator template-id"] # [doc = "                        ::= dn <destructor-name>               # destructor or pseudo-destructor;"] # [doc = "                                                               # e.g. ~X or ~X<N-1>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum BaseUnresolvedName { # [doc = " An unresolved name."] Name (SimpleId) , # [doc = " An unresolved function or template function name."] Operator (OperatorName , Option < TemplateArgs >) , # [doc = " An unresolved destructor name."] Destructor (DestructorName) , }
    };
}

BaseUnresolvedName!();