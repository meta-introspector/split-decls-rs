macro_rules! deps {
    () => {
        SubSectionRequirement!();
    };
}

macro_rules! BASE_PARAMETER {
    () => {
        deps!();
        const BASE_PARAMETER : Option < SubSectionRequirement > = Some (SubSectionRequirement :: Parameter ("base")) ;
    };
}

BASE_PARAMETER!();