macro_rules! deps {
    () => {
        FieldInfo!();
    };
}

macro_rules! CsFold {
    () => {
        deps!();
        # [doc = " The function passed to `cs_fold` is called repeatedly with a value of this"] # [doc = " type. It describes one part of the code generation. The result is always an"] # [doc = " expression."] pub (crate) enum CsFold < 'a > { # [doc = " The basic case: a field expression for one or more selflike args. E.g."] # [doc = " for `PartialEq::eq` this is something like `self.x == other.x`."] Single (& 'a FieldInfo) , # [doc = " The combination of two field expressions. E.g. for `PartialEq::eq` this"] # [doc = " is something like `<field1 equality> && <field2 equality>`."] Combine (Span , Box < Expr > , Box < Expr >) , Fieldless , }
    };
}

CsFold!();