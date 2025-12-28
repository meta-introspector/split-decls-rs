macro_rules! deps {
    () => {
        Definition!();
        Label!();
    };
}

macro_rules! macro_34 {
    () => {
        deps!();
        impl_from ! (Field , Module , Function , Adt , Variant , Const , Static , Trait , TypeAlias , BuiltinType , Local , GenericParam , Label , Macro , ExternCrateDecl for Definition) ;
    };
}

macro_34!()