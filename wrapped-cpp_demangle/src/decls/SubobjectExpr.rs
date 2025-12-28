macro_rules! deps {
    () => {
        Expression!();
    };
}

macro_rules! SubobjectExpr {
    () => {
        deps!();
        # [doc = " The subobject expression production."] # [doc = ""] # [doc = " <expression> ::= so <referent type> <expr> [<offset number>] <union-selector>* [p] E"] # [doc = " <union-selector> ::= _ [<number>]"] # [doc = ""] # [doc = " Not yet in the spec: https://github.com/itanium-cxx-abi/cxx-abi/issues/47"] # [doc = " But it has been shipping in clang for some time."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct SubobjectExpr { ty : TypeHandle , expr : Box < Expression > , offset : isize , }
    };
}

SubobjectExpr!();