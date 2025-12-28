macro_rules! TypeOrConst {
    () => {
        # [derive (Debug)] enum TypeOrConst { Either (ast :: TypeArg) , Const (ast :: ConstArg) , }
    };
}

TypeOrConst!()