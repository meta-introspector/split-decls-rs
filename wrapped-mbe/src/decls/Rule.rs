macro_rules! deps {
    () => {
        MetaTemplate!();
    };
}

macro_rules! Rule {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] struct Rule { lhs : MetaTemplate , rhs : MetaTemplate , }
    };
}

Rule!()