macro_rules! deps {
    () => {
        MangledName!();
    };
}

macro_rules! ExprPrimary {
    () => {
        deps!();
        # [doc = " The `<expr-primary>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <expr-primary> ::= L <type> <value number> E                        # integer literal"] # [doc = "                ::= L <type> <value float> E                         # floating literal"] # [doc = "                ::= L <string type> E                                # string literal"] # [doc = "                ::= L <nullptr type> E                               # nullptr literal (i.e., \"LDnE\")"] # [doc = "                ::= L <pointer type> 0 E                             # null pointer template argument"] # [doc = "                ::= L <type> <real-part float> _ <imag-part float> E # complex floating point literal (C 2000)"] # [doc = "                ::= L <mangled-name> E                               # external name"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum ExprPrimary { # [doc = " A type literal."] Literal (TypeHandle , usize , usize) , # [doc = " An external name."] External (MangledName) , }
    };
}

ExprPrimary!();