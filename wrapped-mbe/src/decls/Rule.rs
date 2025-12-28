macro_rules! Rule {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] struct Rule { lhs : MetaTemplate , rhs : MetaTemplate , }
    };
}

Rule!()